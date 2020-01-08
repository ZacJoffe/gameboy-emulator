use std::io;
use std::io::prelude::*;
use std::env;
use std::fs::File;

mod cpu;
mod flags;
mod instructions;
mod memory_bus;
mod registers;

use cpu::CPU;

fn main() -> io::Result<()> {
    let mut cpu = CPU::new();
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


    println!("Hello, world!");

    Ok(())
}
