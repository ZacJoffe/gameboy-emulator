struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8
}

impl Registers {
    fn get_bc(&self) -> u16 { (self.b as u16) << 8 | self.c as u16 }

    fn set_bc(&mut self, value: u16) {
        let b = ((value & 0xff00) >> 8) as u8;
        let c = (value & 0x00ff) as u8;

        self.b = b;
        self.c = c;
    }
}
