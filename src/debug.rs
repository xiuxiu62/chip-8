use std::io::{stdin, stdout, Read, Write};

use crate::NUM_REGISTERS;

pub fn debug(
    pc: usize,
    instr: u16,
    code: u16,
    x: usize,
    y: usize,
    n: u16,
    nn: u16,
    nnn: u16,
    registers: [u8; NUM_REGISTERS],
    idx: u16,
) {
    println!("");
    println!("L{:03x}:  {}", pc - 2, debug_instr(code, x, y, n, nn, nnn));
    println!("instr:       0x{:04x}", instr);
    println!(
        "V0: 0x{:04x}  V1: 0x{:04x}  V2: 0x{:04x}",
        registers[0], registers[1], registers[2]
    );
    println!(
        "V3: 0x{:04x}  V4: 0x{:04x}  V5: 0x{:04x}",
        registers[3], registers[4], registers[5]
    );
    println!(
        "V6: 0x{:04x}  V7: 0x{:04x}  V5: 0x{:08x}",
        registers[6], registers[7], registers[8]
    );
    println!(
        "V9: 0x{:04x}  VA: 0x{:04x}  VB: 0x{:08x}",
        registers[9], registers[10], registers[11]
    );
    println!(
        "VC: 0x{:04x}  VD: 0x{:04x}  VE: 0x{:08x}",
        registers[12], registers[13], registers[14]
    );
    println!(
        "VC: 0x{:04x}                I: 0x{:04x}",
        registers[15], idx
    );
    // println!("code:        0x{:04x}", code);
    // println!("x:           0x{:04x} ({})", x, x);
    // println!("y:           0x{:04x} ({})", y, y);
    // println!("n:           0x{:04x} ({})", n, n);
    // println!("nn:          0x{:04x} ({})", nn, nn);
    // println!("nnn:         0x{:04x} ({})", nnn, nnn);
    pause();
}

pub fn debug_instr(code: u16, x: usize, y: usize, n: u16, nn: u16, nnn: u16) -> String {
    match code {
        // 00E0 - clear screen
        0x0000 => format!("CLS"),
        // 1NNN - jump
        0x1000 => format!("JMP 0x{:04x}", nnn),
        // 6XNN - set register VX to NN
        0x6000 => format!("LD V{}, 0x{:04x}", x, nn),
        // 7XNN - add NN to register VX
        0x7000 => format!("ADD V{}, 0x{:04x}", x, nn),
        0x8000 => {
            match n {
                // 8XY0 - VX := VY
                0 => format!("LD V{}, V{}", x, y),
                // 8XY1 - VX := VX OR VY
                1 => format!("OR V{}, V{}", x, y),
                // 8XY2 - VX := VX AND VY
                2 => format!("AND V{}, V{}", x, y),
                // 8XY3 - VX := VX XOR VY
                3 => format!("XOR V{}, V{}", x, y),
                // 8XY4 - ADD VX, VY
                4 => format!("ADD V{}, 0x{:04x}", x, y),
                // Default
                _ => format!(""),
            }
        }
        // ANNN - set index register to NNN
        0xA000 => format!("LD I, 0x{:04x}", nnn),
        // DXYN - display/draw
        0xD000 => format!("DRW V{}, V{}, 0x{:04x}", x, y, n),
        _ => format!(""),
    }
    .to_string()
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
