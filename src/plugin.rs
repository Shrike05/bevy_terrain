use crate::systems::*;
use crate::*;
use bevy::prelude::*;
use bitmaps::Bitmap;

pub struct TerrainPlugin {
    chunk_size: usize,
}

impl TerrainPlugin {
    pub fn new(chunk_size: usize) -> Self {
        TerrainPlugin { chunk_size }
    }
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        CHUNK_SIZE.set(self.chunk_size);

        app.insert_resource(BuildabilityMap { map: Bitmap::new() });
        app.insert_resource(HoveredTile {
            pos: GridPos::new(0, 0),
            hovering: false,
        });
        app.add_systems(Startup, spawn_terrain);
        app.add_message::<TileClickedMessage>();
    }
}
