# Installation
1) Install Rust Compiler
2) Run "rustup target add thumb7em-none-eabihf" (Cortex-M4 with FPU)
3) Run "cargo install probe-rs-tool" (needs cmake installed)
4) USB Driver installed for flasher probe
5) (Optional) You might need to change USB Driver to WinUSB via [Zadig](https://zadig.akeo.ie/)


Now you can just use cargo run to build and flash the program
