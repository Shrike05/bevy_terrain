use crate::*;
use bevy::prelude::*;
use bitmaps::Bitmap;

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HoveredTile {
    pub pos: GridPos,
    pub hovering: bool,
}

#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TileClickedMessage {
    pub pos: GridPos,
}

#[derive(Resource, Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct BuildabilityMap {
    pub map: Bitmap<16>,
}

impl TileClickedMessage {
    pub fn new(pos: GridPos) -> TileClickedMessage {
        TileClickedMessage { pos }
    }

    pub fn get_pos(&self) -> &GridPos {
        &self.pos
    }
}

impl BuildabilityMap {
    fn pos_to_index(&self, x: u32, y: u32) -> usize {
        y as usize * get_chunk_size() + x as usize
    }

    pub fn get(&self, x: u32, y: u32) -> bool {
        if x >= get_chunk_size() as u32 || y >= get_chunk_size() as u32 {
            return false;
        }
        self.map.get(self.pos_to_index(x, y))
    }

    pub fn overlaps(&self, tiles: &[GridPos]) -> bool {
        tiles.iter().any(|tile| self.get(tile.x, tile.y))
    }

    pub fn set_real(&mut self, pos: GridPos, val: bool) -> Result<()> {
        let index = self.pos_to_index(pos.x, pos.y);

        if index >= get_chunk_sq() {
            error!("The position is outside the bounds of the bitmap")
        }
        self.map.set(index, val);

        Ok(())
    }
}

impl std::fmt::Display for BuildabilityMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = "".to_string();
        for i in 0..get_chunk_sq() {
            let a = self.map.get(i);

            res += if i % get_chunk_size() == 0 { "\n" } else { "" };
            res += if a { "X" } else { "#" };
        }
        write!(f, "{}", res)
    }
}
