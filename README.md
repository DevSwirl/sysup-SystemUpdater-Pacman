# sysup

Simple Rust CLI tool for updating Arch-based Linux systems using pacman.

Install the binary in /usr/local/bin and run `sysup` from anywhere.

## Requirements

- Arch Linux or Arch-based distro
- pacman
- Rust (cargo, rustc)
- sudo

Install Rust if needed:

sudo pacman -S rust

## Installation

Clone the repository:

git clone https://github.com/DevSwirl/sysup-SystemUpdater-Pacman.git


cd sysup-SystemUpdater-Pacman


cd sysup


cd src



Build the project:#
rustc sysup.rs

Transfer file for system-wide use:
sudo mv /home/{YOUR_NAME}/Desktop/sysup-SystemUpdater-Pacman/sysup/src/sysup /usr/local/bin/


## Usage

Run:
sysup

You will be prompted for your sudo password.

## Development

Run without installing:
While sitting in sysup directory run -
cargo run

## Notes

Arch Linux only.

Once Executable is in /usr/local/bin/ , sysup can be run from anywhere!

## License

MIT
