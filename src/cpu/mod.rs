// CPSR control bits
const CPSR_M0: u32 = 1;
const CPSR_M1: u32 = 1 << 1;
const CPSR_M2: u32 = 1 << 2;
const CPSR_M3: u32 = 1 << 3;
const CPSR_M4: u32 = 1 << 4;
const CPSR_T: u32 = 1 << 5;
const CPSR_F: u32 = 1 << 6;
const CPSR_I: u32 = 1 << 7;

// CPSR condition code flags
const CPSR_V: u32 = 1 << 28;
const CPSR_C: u32 = 1 << 29;
const CPSR_Z: u32 = 1 << 30;
const CPSR_N: u32 = 1 << 31;

// CPSR modes
const MODE_MASK: u32 = CPSR_M4 | CPSR_M3 | CPSR_M2 | CPSR_M1 | CPSR_M0;

const MODE_USER: u32 = CPSR_M4;
const MODE_FIQ: u32 = CPSR_M4 | CPSR_M0;
const MODE_IRQ: u32 = CPSR_M4 | CPSR_M1;
const MODE_SUPERVISOR: u32 = CPSR_M4 | CPSR_M1 | CPSR_M0;
const MODE_ABORT: u32 = CPSR_M4 | CPSR_M2 | CPSR_M1 | CPSR_M0;
const MODE_UNDEFINED: u32 = CPSR_M4 | CPSR_M3 | CPSR_M1 | CPSR_M0;
const MODE_SYSTEM: u32 = CPSR_M4 | CPSR_M3 | CPSR_M2 | CPSR_M1 | CPSR_M0;

// CPSR states
const STATE_ARM: u32 = 0;
const STATE_THUMB: u32 = CPSR_T;

#[derive(Default, Debug)]
pub struct Cpu {
    lower_regs: [u32; 8],
    upper_regs: [u32; 5],
    fiq_upper_regs: [u32; 5],
    sps: [u32; 6],
    lrs: [u32; 6],
    pc: u32,
    cpsr: u32,
    spsrs: [u32; 5],
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            cpsr: CPSR_I | CPSR_F | MODE_SUPERVISOR,
            ..Default::default()
        }
    }

    // exec_nex fetches and executes the next instruction, returning the number
    // of cycles the instruction took
    pub fn exec_next(&mut self) -> Result<usize, String> {
        Err("Not implemented".to_string())
    }

    fn get_reg(&mut self, reg: Register) -> u32 {
        *self.get_reg_by_mode(reg)
    }

    fn set_reg(&mut self, reg: Register, v: u32) {
        *self.get_reg_by_mode(reg) = v;
    }

    fn state(&mut self) -> State {
        (*self.get_reg_by_mode(Register::CPSR) & CPSR_T).into()
    }

    fn mode(&mut self) -> Mode {
        (*self.get_reg_by_mode(Register::CPSR) & MODE_MASK).into()
    }

    // TODO: This should really not be mut
    fn get_reg_by_mode(&mut self, reg: Register) -> &mut u32 {
        match reg {
            Register::Lo(lo) => &mut self.lower_regs[lo as usize],
            Register::PC => &mut self.pc,
            Register::CPSR => &mut self.cpsr,
            Register::SP => &mut self.sps[self.mode().get_mode_reg_offset()],
            Register::LR => &mut self.lrs[self.mode().get_mode_reg_offset()],
            Register::SPSR => match self.mode() {
                Mode::User | Mode::System => panic!("Invalid mode for reg access"),
                _ => &mut self.spsrs[self.mode().get_mode_reg_offset()],
            },
            Register::Hi(hi) => {
                if let State::Thumb = self.state() {
                    panic!("Invalid mode for reg access");
                };

                if let Mode::Fiq = self.mode() {
                    &mut self.fiq_upper_regs[hi as usize]
                } else {
                    &mut self.upper_regs[hi as usize]
                }
            }
        }
    }
}

pub enum Register {
    Lo(Lo),
    Hi(Hi),
    SP,
    LR,
    PC,
    CPSR,
    SPSR,
}

#[derive(Copy, Clone)]
pub enum Lo {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}

#[derive(Copy, Clone)]
pub enum Hi {
    R8 = 0,
    R9,
    R10,
    R11,
    R12,
}

#[derive(Debug)]
enum State {
    Arm,
    Thumb,
}

impl From<u32> for State {
    #[inline]
    fn from(u: u32) -> State {
        match u {
            STATE_ARM => State::Arm,
            STATE_THUMB => State::Thumb,
            // TODO: panic might need to move to cpu with TryInto?
            _ => panic!("CPSR invalid state"),
        }
    }
}

#[derive(Debug)]
enum Mode {
    User,
    Fiq,
    Irq,
    Supervisor,
    Abort,
    System,
    Undefined,
}

impl Mode {
    #[inline]
    fn get_mode_reg_offset(&self) -> usize {
        match &self {
            Mode::User | Mode::System => 0,
            Mode::Fiq => 1,
            Mode::Supervisor => 2,
            Mode::Abort => 3,
            Mode::Irq => 4,
            Mode::Undefined => 5,
        }
    }
}

impl From<u32> for Mode {
    #[inline]
    fn from(u: u32) -> Mode {
        match u {
            MODE_USER => Mode::User,
            MODE_FIQ => Mode::Fiq,
            MODE_IRQ => Mode::Irq,
            MODE_SUPERVISOR => Mode::Supervisor,
            MODE_ABORT => Mode::Abort,
            MODE_UNDEFINED => Mode::Undefined,
            MODE_SYSTEM => Mode::System,
            // TODO: panic might need to move to cpu with TryInto?
            _ => panic!("CPSR invalid mode"),
        }
    }
}
