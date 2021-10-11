#![allow(dead_code)]

use std::{error::Error, fs::File, io::Read};

use clap::{App, Arg};
use sdl2::{event::Event, keyboard::Keycode};

use crate::{audio::Beep, display::Display, emulator::Chip8, util::hex_to_col};

mod audio;
mod cpu;
mod debug;
mod display;
mod emulator;
mod keyboard;
mod time;
mod util;

// Starting address of user programs
pub const PROGRAM_LOC: usize = 0x200;

pub const NUM_REGISTERS: usize = 16;
pub const RAM_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 64;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_LEN: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

// Default foreground color
pub const DEF_FG_COL: &str = "ABAECB";
pub const DEF_FG: (u8, u8, u8) = (171, 171, 203);
// Default background color
pub const DEF_BG_COL: &str = "101020";
pub const DEF_BG: (u8, u8, u8) = (16, 16, 32);
// Default number of instructions per second
pub const DEF_IPS: u32 = 1000;
// Default screen scale factor
pub const DEF_SCALE: u32 = 10;

const FONTS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("chip-8")
        .version("0.1.0")
        .about("chip-8 emulator")
        .arg(
            Arg::with_name("input")
                .required(true)
                .index(1)
                .help("ROM file to load and run"),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .takes_value(false)
                .help("Run in debug mode. Pauses after each instruction, prints info to stdout"),
        )
        .arg(
            Arg::with_name("scale")
                .short("s")
                .long("scale")
                .takes_value(true)
                .help(&format!("Integer display scale factor, defaults to {} (for 640x320 upscaled resolution)", DEF_SCALE)),
        )
        .arg(
            Arg::with_name("ips")
                .short("i")
                .long("ips")
                .takes_value(true)
                .help(&format!("Emulation speed in instructions per second, defaults to {}", DEF_IPS)),
        )
        .arg(
            Arg::with_name("fgcol")
                .short("c")
                .long("fgcol")
                .takes_value(true)
                .help(&format!("Foreground (on) color as a hex code, defaults to {}", DEF_FG_COL)),
        )
        .arg(
            Arg::with_name("bgcol")
                .short("b")
                .long("bgcol")
                .takes_value(true)
                .help(&format!("Background (off) color as a hex code, defaults to {}", DEF_BG_COL)),
        )
        .get_matches();

    let filename = matches.value_of("input").unwrap();
    // Read ROM
    println!("Reading ROM file: {}", filename);
    let mut f = File::open(filename).unwrap();
    let mut rom = Vec::new();
    f.read_to_end(&mut rom).unwrap();

    // Scaling
    let def_scale: &str = &DEF_SCALE.to_string();
    let scl_str = matches.value_of("scale").unwrap_or(def_scale);
    let scl_res = scl_str.parse::<u32>();
    let mut scale: u32 = DEF_SCALE;
    match scl_res {
        Ok(n) => scale = n,
        Err(e) => println!(
            "The scale ({}) is not a valid unsigned integer, using default: {}",
            scl_str, e
        ),
    }

    // Emulation speed
    let default_ips: &str = &DEF_IPS.to_string();
    let ips_str = matches.value_of("ips").unwrap_or(default_ips);
    let ips_res = ips_str.parse::<u32>();
    let mut ips: u32 = DEF_IPS;
    match ips_res {
        Ok(n) => ips = n,
        Err(e) => println!(
            "The ips ({}) is not a valid unsigned integer, using default: {}",
            ips_str, e
        ),
    }
    let instruction_time_ns: u128 = (1e9 as u128 / ips as u128) as u128;

    // Foreground color
    let fg_str = matches.value_of("fgcol").unwrap_or(DEF_FG_COL);
    let fg = hex_to_col(fg_str);
    let fgcol = match fg {
        Ok(fgcol) => fgcol,
        Err(error) => {
            println!("{}", error);
            DEF_FG
        }
    };

    // Background color
    let bg_str = matches.value_of("bgcol").unwrap_or(DEF_BG_COL);
    let bg = hex_to_col(bg_str);
    let bgcol = match bg {
        Ok(bgcol) => bgcol,
        Err(error) => {
            println!("{}", error);
            DEF_BG
        }
    };

    // Start time
    let start: u128 = time::time_nanos();

    println!("chip-8 starting");

    // Init SDL2
    let sdl_context = sdl2::init().unwrap();

    // Create the display
    let mut display = Display::new(&sdl_context, "R-CHIP-8", scale, fgcol, bgcol);

    // Create audio beep
    let beep = Beep::new(&sdl_context);

    // Create the machine
    let debug_mode = matches.occurrences_of("debug") > 0;
    println!("Debug: {}", debug_mode);
    let mut chip8 = Chip8::new(rom, FONTS, start, instruction_time_ns, debug_mode);

    // Main loop
    'mainloop: loop {
        let t: u128 = time::time_nanos();

        // Event loop
        for event in display.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::CapsLock),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }

        // Run the machine
        chip8.cycle(t, &mut display.event_pump);

        // Clear/update display if needed
        if chip8.display_clear_flag {
            display.clear();
        }
        if chip8.display_update_flag {
            display.render(chip8.display);
        }

        // Play/pause the beep
        if chip8.beep_flag {
            beep.play();
        } else {
            beep.pause();
        }
    }
    Ok(())
}
