#[derive(Copy, Clone)]
enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixelValue; 8]; 8];

enum Mode {
    HBlank,
    VBlank,
    Oam,
    Transfer
}

pub struct GPU {
    vram: [u8; 0x2000],
    oam: [u8; 0xa0],
    tile_set: [Tile; 384],
    pub LCDC: u8,
    pub STAT: u8,
    pub SCY: u8,
    pub SCX: u8,
    pub LY: u8,
    pub LYC: u8,
    pub WY: u8,
    pub WX: u8,
    pub BGP: u8,
    pub OBP0: u8,
    pub OBP1: u8
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            vram: [0; 0x2000],
            oam: [0; 0xa0],
            tile_set: [[[TilePixelValue::Zero; 8]; 8]; 384],
            LCDC: 0,
            STAT: 0,
            SCY: 0,
            SCX: 0,
            LY: 0,
            LYC: 0,
            WY: 0,
            WX: 0,
            BGP: 0,
            OBP0: 0,
            OBP1: 0
        }
    }

    pub fn set_vram(&mut self, address: u16, new_byte: u8) {
        self.vram[address as usize] = new_byte;
    }

    pub fn set_oam(&mut self, address: u16, new_byte: u8) {
        self.oam[address as usize] = new_byte;
    }

    fn get_mode(&self) -> Mode {
        match self.STAT & 0x3 {
            0 => Mode::HBlank,
            1 => Mode::VBlank,
            2 => Mode::Oam,
            3 => Mode::Transfer
        }
    }

    fn set_mode(&mut self, mode: Mode) {
        self.STAT |= mode as u8;
    }

    // get the msb as a bool
    fn display_enabled(&self) -> bool {
        ((self.LCDC & 0x80) >> 7) != 0
    }
}
