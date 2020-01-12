pub const BIOS_START: u16 = 0x00;
pub const BIOS_END: u16 = 0xff;

pub const ROM_BANK_0_START: u16 = 0x0000;
pub const ROM_BANK_0_END: u16 = 0x3fff;

pub const ROM_BANK_N_START: u16 = 0x4000;
pub const ROM_BANK_N_END: u16 = 0x7fff;

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9fff;

pub const ERAM_START: u16 = 0xa000;
pub const ERAM_END: u16 = 0xbfff;

pub const WRAM_START: u16 = 0xc000;
pub const WRAM_END: u16 = 0xdfff;

pub const ECHO_START: u16 = 0xe000;
pub const ECHO_END: u16 = 0xfdff;

pub const OAM_START: u16 = 0xfe00;
pub const OAM_END: u16 = 0xfe9f;

pub const IO_START: u16 = 0xff00;
pub const IO_END: u16 = 0xff7f;

pub const HRAM_START: u16 = 0xff80;
pub const HRAM_END: u16 = 0xfffe;

pub const IE_REGISTER: u16 = 0xffff;
