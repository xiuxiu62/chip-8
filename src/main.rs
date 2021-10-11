mod cpu;

use cpu::CPU;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cpu = CPU::new();

    cpu.set_opcode(0x8014)?; // add
    cpu.set_register(0, 0x00)?;
    cpu.set_register(1, 0x01)?;
    // cpu.set_opcode(0x8015)?;

    // cpu.execute()?;
    cpu.run()?;
    cpu.run()?;

    // cpu.execute()?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() -> Result<(), Box<dyn Error>> {
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
