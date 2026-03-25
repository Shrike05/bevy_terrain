# Usage
## Initialization
```rs
use bevy_terrain::*;

let mut app = App::new();
app.add_plugins(DefaultPlugins); //Dependency

let chunk_size: usize = 16;
app.add_plugins(TerrainPlugin::new(chunk_size));
```

## Click Event
```rs
use bevy::terrain::*;

pub fn tile_clicked_event(
    mut ev: MessageReader<TileClickedMessage>,
) {
    for build_ev in ev.read() {
       println!("{:?}", *build_ev.get_pos());
    }
}
```
