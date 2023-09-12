mod cpu;
mod gba;
mod ppu;

use gba::Gba;

fn main() {
    let mut gba = Gba::new();
    println!("{:?}", gba);
}
