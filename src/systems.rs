use crate::*;
use bevy::prelude::*;

pub fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Plane3d::new(Vec3::new(0., 1., 0.), Vec2::new(0.5, 0.5)));
    let material = materials.add(Color::srgb(0.25, 0.25, 0.25));
    let hover_material = materials.add(Color::srgb(0.1, 0.1, 0.1));

    for i in 0..get_chunk_sq() {
        let pos = grid_to_world(&index_to_grid(i as u32));

        commands
            .spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                worldpos_to_transform(pos),
            ))
            .observe(update_tile::<Pointer<Over>>(hover_material.clone(), true))
            .observe(update_tile::<Pointer<Out>>(material.clone(), false))
            .observe(tile_clicked::<Pointer<Click>>());
    }
}

pub fn update_tile<E: EntityEvent>(
    new_material: Handle<StandardMaterial>,
    hovering: bool,
) -> impl Fn(On<E>, Query<(&mut MeshMaterial3d<StandardMaterial>, &Transform)>, ResMut<HoveredTile>)
{
    move |event, mut query, mut hovered_tile| {
        if let Ok((mut material, transform)) = query.get_mut(event.event_target()) {
            material.0 = new_material.clone();
            hovered_tile.pos = world_to_grid(&transform.translation);
            hovered_tile.hovering = hovering;
        }
    }
}

pub fn tile_clicked<E: EntityEvent>()
-> impl Fn(On<E>, Query<&Transform>, MessageWriter<TileClickedMessage>) {
    move |event, mut query, mut msg| {
        if let Ok(transform) = query.get_mut(event.event_target()) {
            msg.write(TileClickedMessage::new(world_to_grid(
                &transform.translation,
            )));
        }
    }
}
