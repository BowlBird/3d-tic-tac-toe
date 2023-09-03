# 3D Tic Tac Toe
A Network based 3D Tic Tac Toe game!

### Written in Vue & Tauri

## Building

To install rust/cargo
run: 
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
or go to 
```
https://www.rust-lang.org/learn/get-started
```
on windows.
and go through the installer.

If you do not have NPM installed, you will need to download it [here](https://nodejs.org/en/download).

After cloning the project, in the folder you will need to run 
```
cargo install tauri-cli
```
to install tauri
and
```
npm install
```
to install all vue packages.

then in order to build run 
```
cargo tauri build
```

## Development

If you need to install cargo/npm/other packages, follow build instructions.

Then run 
```
cargo tauri dev
```