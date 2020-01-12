pub struct GPU {
    pub vram: [u8; 0x2000],
    pub oam: [u8; 0xa0]
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            vram: [0; 0x2000],
            oam: [0; 0xa0]
        }
    }
}
