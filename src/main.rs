mod cli;
mod cpu;
mod gba;
mod ppu;

use clap::Parser;
use cli::Command;

fn main() {
    let cmd = Command::parse();
    cmd.run()
}
