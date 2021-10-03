mod error;

use std::fmt::Display;

use error::{ProcessorError, ProcessorResult};

pub struct Processor {
    opcode: u16,
    registers: [u8; 2],
}

impl Processor {
    pub fn new() -> Self {
        Self {
            opcode: 0x0000,
            registers: [0x0000; 2],
        }
    }

    pub fn run(&mut self) -> ProcessorResult<()> {
        for _ in (0..100).into_iter() {
            self.execute()?;
            self.print()?;
            print!(" ");
            // sleep(Duration::from_millis(500))
        }
        Ok(())
    }

    pub fn execute(&mut self) -> ProcessorResult<()> {
        let opcode = self.get_opcode();
        let (c, x, y, d) = as_nibbles(opcode.clone());

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add(x, y)?,
            _ => unimplemented!("opcode {:04x}", opcode),
        }

        Ok(())
    }

    fn add(&mut self, x: u8, y: u8) -> ProcessorResult<()> {
        self.set_register(
            x as usize,
            self.get_register(x as usize)?.as_ref() + self.get_register(y as usize)?.as_ref(),
        )?;
        Ok(())
    }

    fn print(&self) -> ProcessorResult<()> {
        print!("{}", self.get_register(0)?);
        Ok(())
    }

    pub fn set_opcode(&mut self, opcode: u16) -> ProcessorResult<()> {
        if opcode == 0xffff {
            return Err(ProcessorError::InvalidOpcode(opcode));
        }

        self.opcode = opcode;
        Ok(())
    }

    pub fn get_opcode(&self) -> &u16 {
        &self.opcode
    }

    fn register_exists(&self, i: usize) -> bool {
        self.registers.len() > i
    }

    pub fn set_register(&mut self, i: usize, val: u8) -> ProcessorResult<()> {
        if !self.register_exists(i) {
            return Err(ProcessorError::InvalidRegister(i));
        }

        self.registers[i] = val;
        Ok(())
    }

    pub fn get_register(&self, i: usize) -> ProcessorResult<Box<u8>> {
        if !self.register_exists(i) {
            return Err(ProcessorError::InvalidRegister(i));
        }

        Ok(Box::new(self.registers[i]))
    }
}

impl Display for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [r1, r2] = self.registers;
        write!(f, "OP: 0x{:x}\nR1: 0x{:x} R2: 0x{:x}", self.opcode, r1, r2)
    }
}

fn as_nibbles(opcode: u16) -> (u8, u8, u8, u8) {
    let c = ((opcode & 0xF000) >> 12) as u8;
    let x = ((opcode & 0x0F00) >> 8) as u8;
    let y = ((opcode & 0x00F0) >> 4) as u8;
    let d = ((opcode & 0x000F) >> 0) as u8;

    (c, x, y, d)
}
