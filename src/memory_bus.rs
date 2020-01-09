// abstract memory into its logical parts instead of one big array
// currently do not have an implementation for echo ram
pub struct MemoryBus {
    bios: [u8; 0xff],
    rom_bank0: [u8; 0x4000],
    rom_bankn: [u8; 0x4000],
    vram: [u8; 0x2000],
    eram: [u8; 0x2000],
    wram: [u8; 0x2000],
    oam: [u8; 0xa0],
    hram: [u8; 0x7f],
    // memory: [u8; 0xffff],
}

impl MemoryBus {
    pub fn new(rom: Vec<u8>, game: Vec<u8>) -> MemoryBus {
        let bios = [0; 0xff];
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
            vram: [0; 0x2000],
            eram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xa0],
            hram: [0, 0x7f]
        }
    }

    // broken
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    // broken
    pub fn set_byte(&mut self, address: u16, new_byte: u8) {
        self.memory[address as usize] = new_byte;
    }
}
