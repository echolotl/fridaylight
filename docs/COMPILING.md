# Compiling Fridaylight

## Requirements
- [Bun](https://bun.sh/)
- [MSVC C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- [Rust](https://www.rust-lang.org/)

*This document will go over dependency installation for Windows specifically, as that's what this program currently supports. For information about how to compile for Mac or Linux, you can refer to the [official Tauri Quick Start tutorial](https://v2.tauri.app/start/prerequisites/).*

## Instructions

### 1. Install MSVC C++ Build Tools:
After downloading and running the installer from the link above, select the "Desktop development with C++" option. This will install the neccesary tools to build the program.

![Visual Studio C++ Build Tools installer screenshot](https://github.com/user-attachments/assets/f2a678c7-3929-4463-8cb9-bf809fe447fc)

### 2. Install Rust
To install Rust, run this command in your terminal:
```pwsh
winget install --id Rustlang.Rustup
```
This will install `rustup`. After going through the installer, set the correct toolchain (MSVC) using:
```pwsh
rustup default stable-msvc
```

You will need to restart your terminal, and possibly your system, if neccesary.

# 3. Install Bun and project dependencies

To install Bun, run this command in your terminal:
```pwsh
powershell -c "irm bun.sh/install.ps1 | iex"
```

After that is installed, run this command in the root directory of the project:
```pwsh
bun i
```

## Running the program

You can run it from the terminal directly using a dev server with:
```pwsh
bun run tauri dev
```
or build an installer using:
```pwsh
bun run tauri build
```



