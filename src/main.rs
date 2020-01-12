use std::io;
use std::io::prelude::*;
use std::env;
use std::fs::File;

mod cpu;
mod flags;
mod instructions;
mod memory_bus;
mod registers;
mod memory_map;
mod gpu;

use cpu::CPU;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Must give bios and game rom!")
    }

    // open files from args
    // first arg is the path to bios
    // second arg is the path to game rom
    let mut bios = File::open(String::from(&args[1]))?;
    let mut game = File::open(String::from(&args[2]))?;

    // load bytes of file into a buffer
    let mut bios_buffer = Vec::new();
    bios.read_to_end(&mut bios_buffer)?;

    let mut game_buffer = Vec::new();
    game.read_to_end(&mut game_buffer)?;

    if bios_buffer.len() != 256 {
        panic!("BIOS is the wrong size!");
    }


    let mut cpu = CPU::new(bios_buffer, game_buffer);

    println!("Hello, world!");

    Ok(())
}
