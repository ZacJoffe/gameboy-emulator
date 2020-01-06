pub struct MemoryBus {
    memory: [u8; 0xffff],
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            memory: [0; 0xffff],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn set_byte(&mut self, address: u16, new_byte: u8) {
        self.memory[address as usize] = new_byte;
    }
}
