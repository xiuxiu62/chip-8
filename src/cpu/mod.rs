mod error;

use std::fmt::Display;

use error::{CPUError, CPUResult};

pub struct CPU {
    opcode: u16,
    registers: [u8; 2],
}

impl CPU {
    pub fn new() -> Self {
        Self {
            opcode: 0x0000,
            registers: [0x0000; 2],
        }
    }

    pub fn run(&mut self) -> CPUResult<()> {
        // for _ in (0..255).into_iter() {
        //     self.execute()?;
        //     self.print(Some(" "))?;

        //     thread::sleep(Duration::from_millis(5))
        // }
        self.execute()?;
        self.print(Some("\n"))?;

        Ok(())
    }

    pub fn execute(&mut self) -> CPUResult<()> {
        let (c, x, y, d) = self.get_nibbles();

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add(x, y)?,
            _ => eprintln!("opcode \"{:04x}\" not implemented", self.get_opcode()),
        }

        Ok(())
    }

    fn add(&mut self, x: u8, y: u8) -> CPUResult<()> {
        self.set_register(
            x as usize,
            self.get_register(x as usize)?.as_ref() + self.get_register(y as usize)?.as_ref(),
        )?;
        Ok(())
    }

    fn print(&self, delim: Option<&str>) -> CPUResult<()> {
        print!(
            "{}{}",
            self.get_register(0)?,
            match delim {
                Some(val) => val,
                None => "",
            }
        );
        Ok(())
    }

    pub fn set_opcode(&mut self, opcode: u16) -> CPUResult<()> {
        if opcode == 0xffff {
            return Err(CPUError::InvalidOpcode(opcode));
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

    pub fn set_register(&mut self, i: usize, val: u8) -> CPUResult<()> {
        if !self.register_exists(i) {
            return Err(CPUError::InvalidRegister(i));
        }

        self.registers[i] = val;
        Ok(())
    }

    pub fn get_register(&self, i: usize) -> CPUResult<Box<u8>> {
        if !self.register_exists(i) {
            return Err(CPUError::InvalidRegister(i));
        }

        Ok(Box::new(self.registers[i]))
    }

    fn get_nibbles(&self) -> (u8, u8, u8, u8) {
        let c = ((self.opcode & 0xF000) >> 12) as u8;
        let x = ((self.opcode & 0x0F00) >> 8) as u8;
        let y = ((self.opcode & 0x00F0) >> 4) as u8;
        let d = ((self.opcode & 0x000F) >> 0) as u8;
        (c, x, y, d)
    }
}

impl Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [r1, r2] = self.registers;
        write!(f, "OP: 0x{:x}\nR1: 0x{:x} R2: 0x{:x}", self.opcode, r1, r2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() -> CPUResult<()> {
        let mut cpu = CPU::new();
        cpu.set_opcode(0x8014)?; // add
        cpu.set_register(0, 0x00)?;
        cpu.set_register(1, 0x01)?;
        // cpu.set_opcode(0x8015)?;

        for i in 1..=5 {
            cpu.execute()?;
            assert_eq!(cpu.get_register(0)?.as_ref(), &(i as u8));
        }

        Ok(())
    }
}
