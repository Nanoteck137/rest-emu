pub struct Mmu {
    ram: Vec<u8>,
}

impl Mmu {
    pub fn new(size: usize) -> Self {
        Self {
            ram: vec![0; size],
        }
    }

    pub fn write_u8(&mut self, addr: u64, value: u8) {
        self.ram[addr as usize] = value;
    }

    pub fn read_u8(&self, addr: u64) -> u8{
        self.ram[addr as usize]
    }

    pub fn write_u16(&mut self, addr: u64, value: u16) {
        self.write_u8(addr, (value & 0xff) as u8);
        self.write_u8(addr + 1, ((value >> 8) & 0xff) as u8);
    }

    pub fn read_u16(&self, addr: u64) -> u16 {
        self.read_u8(addr) as u16 | (self.read_u8(addr + 1) as u16) << 8
    }

    pub fn write_u32(&mut self, addr: u64, value: u32) {
        self.write_u8(addr + 0, ((value >> 0) & 0xff) as u8);
        self.write_u8(addr + 1, ((value >> 8) & 0xff) as u8);
        self.write_u8(addr + 2, ((value >> 16) & 0xff) as u8);
        self.write_u8(addr + 3, ((value >> 24) & 0xff) as u8);
    }

    pub fn read_u32(&self, addr: u64) -> u32 {
        self.read_u8(addr) as u32 |
            (self.read_u8(addr + 1) as u32) << 8 |
            (self.read_u8(addr + 2) as u32) << 16 |
            (self.read_u8(addr + 3) as u32) << 24
    }

    pub fn write_u64(&mut self, addr: u64, value: u64) {
        self.write_u8(addr + 0, ((value >> 0) & 0xff) as u8);
        self.write_u8(addr + 1, ((value >> 8) & 0xff) as u8);
        self.write_u8(addr + 2, ((value >> 16) & 0xff) as u8);
        self.write_u8(addr + 3, ((value >> 24) & 0xff) as u8);

        self.write_u8(addr + 4, ((value >> 32) & 0xff) as u8);
        self.write_u8(addr + 5, ((value >> 40) & 0xff) as u8);
        self.write_u8(addr + 6, ((value >> 48) & 0xff) as u8);
        self.write_u8(addr + 7, ((value >> 56) & 0xff) as u8);
    }

    pub fn read_u64(&self, addr: u64) -> u64 {
        self.read_u8(addr) as u64 |
            (self.read_u8(addr + 1) as u64) << 8 |
            (self.read_u8(addr + 2) as u64) << 16 |
            (self.read_u8(addr + 3) as u64) << 24 |
            (self.read_u8(addr + 4) as u64) << 32 |
            (self.read_u8(addr + 5) as u64) << 40 |
            (self.read_u8(addr + 6) as u64) << 48 |
            (self.read_u8(addr + 7) as u64) << 56
    }
}
