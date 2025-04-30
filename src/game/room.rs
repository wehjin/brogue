use crate::game::constants::TILE_INTERVAL;
use crate::game::grid::Tile;
use crate::game::resources::Room;
use bevy::prelude::*;

pub struct RoomPlugin;
impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Room {
            south: 7,
            north: 12,
            west: 30,
            east: 50,
        });
        app.add_systems(Startup, spawn_rooms);
    }
}

pub fn spawn_rooms(
    room: Res<Room>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // floor base
    let floor_material = materials.add(Color::WHITE);
    const WIDTH: f32 = TILE_INTERVAL.x * 0.78;
    const HEIGHT: f32 = TILE_INTERVAL.y * 0.91;
    for gy in room.south_to_north() {
        for gx in room.west_to_east() {
            let tile = Tile::new(gx, gy);
            commands.spawn((
                Mesh3d(meshes.add(Rectangle::new(WIDTH, HEIGHT))),
                MeshMaterial3d(floor_material.clone()),
                Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
                    .with_translation(tile.center()),
            ));
        }
    }
}
