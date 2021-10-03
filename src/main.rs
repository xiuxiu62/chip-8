mod processor;

use processor::Processor;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cpu = Processor::new();

    cpu.set_opcode(0x8014)?;
    cpu.set_register(0, 0x00)?;
    cpu.set_register(1, 0x01)?;

    // cpu.execute()?;
    cpu.run()?;

    // cpu.execute()?;

    Ok(())
}
