# rtxflash

Rust library to flash ham radios for OpenRTX, paired with a minimal CLI interface

## Building under linux

Install Rust toolchain:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Build
```bash
cargo build
```

## Building under Windows

Install MSYS2 from [here](https://www.msys2.org/), open MSYS2 terminal.

Install dependencies:
```bash
curl https://sh.rustup.rs -sSf | sh
# Enter Y, 2 to customize installation, select triple x86_64-pc-windows-gnu, leave others as default
pacman -S git mingw-w64-x86_64-gcc
git clone https://github.com/OpenRTX/rtxflash
# Add Rust tools to path
echo "export PATH=\"\$PATH:\${USERPROFILE}\.cargo\bin\"" >> ~/.bashrc
# Source new bashrc
. ~/.bashrc
```

Build
```bash
cd rtxflash
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu
```
