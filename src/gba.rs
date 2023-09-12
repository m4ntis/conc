use crate::cpu::Cpu;
use crate::ppu::Ppu;

#[derive(Debug)]
pub struct Gba {
    c: Cpu,
    p: Ppu,
}

impl Gba {
    pub fn new() -> Self {
        Self {
            c: Cpu::new(),
            p: Ppu::new(),
        }
    }

    pub fn start(&mut self) {
        loop {
            let cycles = self.c.exec_next();

            for n in 1..cycles.unwrap() {
                self.p.cycle();
            }
        }
    }
}
