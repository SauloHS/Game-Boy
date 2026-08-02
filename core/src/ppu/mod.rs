pub const VRAM_START: u16 = 0x8000;
pub const VRAM_STOP: u16 = 0x9FFF;
const TILE_SET_START: u16 = 0x8000;
const TILE_SET_STOP: u16 = 0x97FF;
const TILE_MAP_START: u16 = 0x9800;
const TILE_MAP_STOP: u16 = 0x9FFF;
const BYTES_PER_TILE: u16 = 16;
const NUM_TILES: usize = 384;

use tile::Tile;
pub struct Ppu {
    tiles: [Tile; NUM_TILES],
}

mod tile;

impl Ppu {
    pub fn new() -> Self {
        Self {
            tiles: [Tile::new(); NUM_TILES],
        }
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        match addr {
            TILE_SET_START..=TILE_SET_STOP => {
                todo!();
            }
            TILE_MAP_START..=TILE_MAP_STOP => {
                todo!();
            }
            _ => {
                unreachable!()
            }
        }
    }

    pub fn write_vram(&mut self, addr: u16, val: u8) {
        match addr {
            TILE_SET_START..=TILE_SET_STOP => {
                todo!();
            }
            TILE_MAP_START..=TILE_MAP_STOP => {
                todo!();
            }
            _ => {
                unreachable!()
            }
        }
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}
