# RustOS-Tpl
A quickstart template for making operating systems with Rust.

# Requirements
This tempalte currently requires:
- xorriso - https://www.gnu.org/software/xorriso/

# Quickstart
**NOTE:** If you have any issues with cargo saying that x86\_64-unknown-none target isn't found, please leave an issue so I can update the README
Windows:
```cmd
rustc build.rs -o build.exe
build                               # Builds your project
```
Linux:
```sh
$ rustc build.rs -o build
$ ./build                           # Builds your project
```
MacOS:
Haven't tried it. It should work tho (similar to linux)

If you have qemu installed, you can also run:
```
build bruh                          # build + run
```
Which will build the iso and also run qemu-system


# Extra
build.rs is a sort of "build script" that has sub commands.
To view them simply do:
```
build help
```
This might come accross as a bit heavy for just a build script and if you wanted to, you could always look at its code and see that most of it is fairly simple.
Run a couple of commands, copy a couple of files and run a couple more commands.

You could also ditch this entire concept and just simply 'simpify' the code - Remove the whole subcommand system and just call to build direct.

# Purpose
This is mainly a template for quickly getting a rust written OS to a point where the OS boots
The template mostly followes things I like with a build system (Things like a simple subcommand system for example).
This may not fit your needs, and thats ok, you can always start from scratch with build.rs, but besides that, everything usualy is the same
(Same linker script, same dependencies, cargo project, etc. etc.)

The template uses limine as its chosen bootloader but you can switch it out if you choose to. Modify the build.rs script as you see fit, add commands, remove commands, simply do whatever you like.
We need a bit of control over how our OS is built for things like linking and project management so thats why we have build.rs but under the hood all of this still uses cargo tho, and if you wanted to switch to just rustc you could always do that
