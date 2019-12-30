use crate::flags::Flags;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: Flags,
    pub h: u8,
    pub l: u8
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: Flags::new(),
            h: 0,
            l: 0
        }
    }

    pub fn get_af(&self) -> u16 { (self.a as u16) << 8 | u8::from(&self.f) as u16 }

    pub fn set_af(&mut self, value: u16) {
        let a = ((value & 0xff00) >> 8) as u8;
        let f = Flags::from((value & 0x00ff) as u8);

        self.a = a;
        self.f = f;
    }

    pub fn get_bc(&self) -> u16 { (self.b as u16) << 8 | self.c as u16 }

    pub fn set_bc(&mut self, value: u16) {
        let b = ((value & 0xff00) >> 8) as u8;
        let c = (value & 0x00ff) as u8;

        self.b = b;
        self.c = c;
    }

    pub fn get_de(&self) -> u16 { (self.d as u16) << 8 | self.e as u16 }

    pub fn set_de(&mut self, value: u16) {
        let d = ((value & 0xff00) >> 8) as u8;
        let e = (value & 0x00ff) as u8;

        self.d = d;
        self.e = e;
    }

    pub fn get_hl(&self) -> u16 { (self.h as u16) << 8 | self.l as u16 }

    pub fn set_hl(&mut self, value: u16) {
        let h = ((value & 0xff00) >> 8) as u8;
        let l = (value & 0x00ff) as u8;

        self.h = h;
        self.l = l;
    }
}
