fn main() {
    println!("Hello, world!");
}

pub struct Cpu {
    pub registers: Registers,
    pub pc: u16,
    pub flags: Flags,
    pub stack: [u16; 7],
    pub memory: [u8; 16384],
    pub data_bus: u8,
    pub address_bus: u16,
}

pub struct Flags {
    pub c: u8,
    pub p: u8,
    pub z: u8,
    pub s: u8,
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}