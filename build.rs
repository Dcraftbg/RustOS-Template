use std::process::Command;
use std::process::ExitCode;
use std::path::{PathBuf,Path};
use std::env::Args;
/// A function for copying all files from some place into a directory
/// Very useful :)
fn copy_all(what: &[&str], to: &str) -> std::io::Result<()> {
    for p in what {
        let p_file = Path::new(p).file_name().expect("Expected to copy file name instead of this");
        let mut top: PathBuf = PathBuf::from(to);
        top.push(p_file);
        std::fs::copy(p,&top).map_err(|e| {
            eprintln!("ERROR: Failed to copy file from {} to {}",p,top);
            e
        })?;
    } 
    Ok(())
}
/// This kind of 'function reports error when it happens, and only signals that an error has occured'
/// is inspired by @Tsoding 
type SUBCMD_RET = Result<(),()>;

/// This just describes a sub command.
/// Its basically just name + desc and a function to run the command (checkout `const COMMANDS`
/// below)
struct Subcmd {
    name: &'static str,
    desc: &'static str,
    run: fn (&mut Env) -> SUBCMD_RET, 
}
/// Very common arguments for commands.
/// - The executable itself (so on windows its ./build.exe and on linux its the full path to build)
/// - The arguments passed to the command
struct Env {
    exe: String,
    args: Args
}
impl Env {
    /// The build command is pretty straightforward. 
    fn build(&mut self) -> SUBCMD_RET {
        // TODO: Pass arguments onto cargo maybe? idk
        // cargo build --target x86_64-unknown-none
        let cargo = Command::new("cargo")
            .arg("build")
            .args(["--target", "x86_64-unknown-none"])
            .spawn()
            .map_err(|e| {
                eprintln!("Executing cargo failed: {}",e);
            })?.wait()
            .map_err(|e| {
                eprintln!("Running cargo failed: {}",e);
            })?;
        if !cargo.success() {
            eprintln!("Cargo exited with non-zero exit code");
            return Err(());
        }

        // Project structure
        /*
        ... (src files + projects)
        final/
            //-- This will get turned into the final OS.iso
            //v
            build/                
                kernel                  // Built binary
                limine-bios.sys
                limine-bios-cd.bin
                limine.cfg 
                limine-uefi-cd.bin
                BOOTX64.EFI
            OS.iso
        */
        let fb = "./final/build/"; 
        std::fs::create_dir_all(fb)
        .map_err(|e| {
            eprintln!("Creating {} failed: {}",fb,e);
        })?;
        // Copy all the files necessary for limine and our kernel into the build folder (^Project structure above)
        copy_all(
            &[
                "target/x86_64-unknown-none/debug/kernel", // TODO: Change this out for when we build for release
                "vendor/limine/limine-bios.sys", "./vendor/limine/limine-bios-cd.bin", "./limine.cfg", "./vendor/limine/limine-uefi-cd.bin",
                "vendor/limine/BOOTX64.EFI"                // NOTE: Separated because BIOSX64 can be switched out depending on the platform ig
            ],
            "./final/build/"
        ).map_err(|e| {
            eprintln!("Copying files failed {}",e);
        })?;
        
        let xorriso = Command::new("xorriso")
                        .args(
                            [
                                "-as", "mkisofs",
                                "-b", "limine-bios-cd.bin",
                                "-no-emul-boot",
                                "-boot-load-size", "4",
                                "-boot-info-table",
                                "--efi-boot", "limine-uefi-cd.bin",
                                "-efi-boot-part",
                                "--efi-boot-image",
                                "./final/build",
                                "-o",
                                "./final/OS.iso"
                            ]
                        ).spawn().map_err(|e| {
                            eprintln!("Executing xorriso failed {}",e)
                        })?.wait().map_err(|e| {
                            eprintln!("Running xorriso failed {}",e)
                        })?;

        if !xorriso.success() {
            eprintln!("Running xorriso failed!");
            return Err(());
        }
        Ok(())
    }
    /// Runs the OS.iso with qemu-system-x86_64
    fn run(&mut self) -> SUBCMD_RET {
        // Run qemu with the following arguments:
        // qemu-system-x86_64 -device isa-debug-exit -cpu max -smp 2 -m 128 -cdrom final/OS.iso
        let qemu = Command::new("qemu-system-x86_64")
                    .args(
                        [
                        "-device", "isa-debug-exit",
                        "-cpu", "max",
                        "-smp", "2",
                        "-m", "128", // Mb
                        "-cdrom", "./final/OS.iso"
                        ])
                    .spawn().map_err(|e| {
                        eprintln!("Executing qemu failed {}",e)
                    })?.wait().map_err(|e| {
                        eprintln!("Running qemu failed {}",e)
                    })?;
        if !qemu.success() {
            println!("NOTE: qemu exited with {:?}",qemu);
        }
        Ok(())
    }
    /// Build + Run
    fn bruh(&mut self) -> SUBCMD_RET {
        self.build()?;
        self.run()?;
        Ok(())
    }
    fn help(&mut self) -> SUBCMD_RET {
        if let Some(cmd) = self.args.next() {
            if let Some(scmd) = COMMANDS.iter().find(|x| x.name == cmd) {
                println!("{} - {}",cmd, scmd.desc);
            } else {
                eprintln!("Unknown subcommand {}",cmd);
                return Err(())
            }
        } else {
            println!("{} (command):",self.exe);
            for cmd in COMMANDS.iter() {
                println!("  {: <8} - {}", cmd.name, cmd.desc);
            }
        }
        Ok(())
    }
}
/// Here is where you define your sub commands
const COMMANDS: &[Subcmd] = &[
    Subcmd { name: "build", run: Env::build, desc: "build the OS (default option)"},
    Subcmd { name: "run"  , run: Env::run  , desc: "run the OS using qemu"},
    Subcmd { name: "bruh" , run: Env::bruh , desc: "build + run"},
    Subcmd { name: "help" , run: Env::help , desc: "shows information on a command or the usage if no command is provided"},
];
fn main() -> ExitCode {
    let mut args = std::env::args();
    let exe = args.next().expect("exe");
    let mut env = Env { exe, args };
    let cmd;
    if let Some(arg) = env.args.next() {
        cmd = arg;
    } else {
        cmd = "build".to_string();
    }
    match COMMANDS.iter().find(|x| x.name == cmd) {
        Some(scmd) => {
            match (scmd.run)(&mut env) {
                Ok(_) => ExitCode::SUCCESS,
                Err(_) => {
                    eprintln!("Running {} failed",cmd);
                    ExitCode::FAILURE
                }
            }
        }
        None => {
            eprintln!("Unknown subcommand: {}",cmd);
            ExitCode::FAILURE
        }
    }

}

// TODO: Do we need to call limine bios-install or is it optional?
// TODO: Custom library for creating isos that doesn't rely on xorriso as an external Command
// OR bundle xorriso as part of ./vendor and call that (Probably not the best tho. Not
// cross-platform if we don't bundle it)
