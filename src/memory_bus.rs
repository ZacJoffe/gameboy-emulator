pub struct MemoryBus {
    memory: [u8; 0xffff]
}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}
