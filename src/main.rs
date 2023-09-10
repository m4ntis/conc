mod cpu;

use cpu::{Cpu, Hi, Lo, Register};

fn main() {
    let mut c = cpu::Cpu::new();
    println!("{:?}", c);
    c.set_reg(Register::SP, 29);
    c.set_reg(Register::PC, 29);
    c.set_reg(Register::Lo(Lo::R0), 29);
    c.set_reg(Register::Hi(Hi::R8), 29);
    println!("{:?}", c);
}
