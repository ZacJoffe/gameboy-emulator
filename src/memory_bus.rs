use crate::gpu::GPU;
use crate::memory_map::*;

// abstract memory into its logical parts instead of one big array
// currently do not have an implementation for echo ram
pub struct MemoryBus {
    bios: [u8; 0xff],
    rom_bank0: [u8; 0x4000],
    rom_bankn: [u8; 0x4000],
    // vram: [u8; 0x2000],
    eram: [u8; 0x2000],
    wram: [u8; 0x2000],
    // oam: [u8; 0xa0],
    hram: [u8; 0x7f],
    // memory: [u8; 0xffff],
    gpu: GPU,
}

impl MemoryBus {
    pub fn new(rom: Vec<u8>, game: Vec<u8>) -> MemoryBus {
        let mut bios = [0; 0xff];
        for (i, &byte) in rom.iter().enumerate() {
            bios[i] = byte;
        }

        // load the game into memory
        let mut bank0 = [0; 0x4000];
        let mut bankn = [0; 0x4000];
        for i in 0..0x4000 {
            bank0[i] = game[i];
            bankn[i] = game[i + 0x4000];
        }

        /*
        let mut bankn = [0; 0x4000];
        for i in 0..0x4000 {
            bankn[i] = game[i + 0x4000];
        }
        */

        MemoryBus {
            bios: bios,
            rom_bank0: bank0,
            rom_bankn: bankn,
            // vram: [0; 0x2000],
            eram: [0; 0x2000],
            wram: [0; 0x2000],
            // oam: [0; 0xa0],
            hram: [0; 0x7f],
            gpu: GPU::new()
        }
    }

    // broken
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            BIOS_START..=BIOS_END => self.bios[address as usize],
            ROM_BANK_0_START..=ROM_BANK_0_END => self.rom_bank0[address as usize],
            ROM_BANK_N_START..=ROM_BANK_N_END => self.rom_bankn[(address - ROM_BANK_N_START) as usize],
            VRAM_START..=VRAM_END => self.gpu.vram[(address - VRAM_START) as usize],
            ERAM_START..=ERAM_END => self.eram[(address - ERAM_START) as usize],
            WRAM_START..=WRAM_END => self.wram[(address - WRAM_START) as usize],
            ECHO_START..=ECHO_END => self.wram[(address - ECHO_START) as usize],
            OAM_START..=OAM_END => self.gpu.vram[(address - OAM_START) as usize],
            IO_START..=IO_END => self.read_io_register(address),
            HRAM_START..=HRAM_END => self.hram[(address - HRAM_START) as usize],
            _ => panic!("Cannot access address 0x{}", address)
        }
    }

    // broken
    pub fn set_byte(&mut self, address: u16, new_byte: u8) {
        match address {
            BIOS_START..=BIOS_END => self.bios[address as usize] = new_byte,
            ROM_BANK_0_START..=ROM_BANK_0_END => self.rom_bank0[address as usize] = new_byte,
            ROM_BANK_N_START..=ROM_BANK_N_END => self.rom_bankn[(address - ROM_BANK_N_START) as usize] = new_byte,
            // VRAM_START..=VRAM_END => self.gpu.vram[(address - VRAM_START) as usize] = new_byte,
            VRAM_START..=VRAM_END => self.gpu.set_vram(address - VRAM_START, new_byte),
            ERAM_START..=ERAM_END => self.eram[(address - ERAM_START) as usize] = new_byte,
            WRAM_START..=WRAM_END => self.wram[(address - WRAM_START) as usize] = new_byte,
            ECHO_START..=ECHO_END => self.wram[(address - ECHO_START) as usize] = new_byte,
            // OAM_START..=OAM_END => self.gpu.oam[(address - OAM_START) as usize] = new_byte,
            OAM_START..=OAM_END => self.gpu.set_oam(address - VRAM_START, new_byte),
            IO_START..=IO_END => self.write_io_register(address, new_byte),
            HRAM_START..=HRAM_END => self.hram[(address - HRAM_START) as usize] = new_byte,
            _ => panic!("Cannot access address 0x{}", address)
        };
    }

    // TODO
    fn read_io_register(&self, address: u16) -> u8 {
        0
    }

    // TODO
    fn write_io_register(&mut self, address: u16, new_byte: u8) {

    }
}
