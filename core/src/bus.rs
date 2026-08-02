use crate::cart::{Cart, ROM_START, ROM_STOP};
use crate::ppu::{Ppu, VRAM_START, VRAM_STOP};

pub struct Bus {
    ram: [u8; 0x10000],
    rom: Cart,
    ppu: Ppu,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            ram: [0; 0x10000],
            rom: Cart::new(),
            ppu: Ppu::new(),
        }
    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        match addr {
            ROM_START..=ROM_STOP => self.rom.read_cart(addr),
            VRAM_START..=VRAM_STOP => self.ppu.read_vram(addr),
            _ => {
                let offset = addr - VRAM_STOP - 1;
                self.ram[offset as usize]
            }
        }
    }

    pub fn write_ram(&mut self, addr: u16, val: u8) {
        match addr {
            ROM_START..=ROM_STOP => {
                self.rom.write_cart(addr, val);
            }
            VRAM_START..=VRAM_STOP => {
                self.ppu.write_vram(addr, val);
            }
            _ => {
                let offset = addr - VRAM_STOP - 1;
                self.ram[offset as usize] = val;
            }
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.rom.load_cart(data);
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}
