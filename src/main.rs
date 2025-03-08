pub mod module_bindings;
pub mod player;
pub mod server_connection;

use bevy::prelude::*;
use player::{Player, PlayerPlugin};
use server_connection::SpaceTimeConnectionPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PlayerPlugin, SpaceTimeConnectionPlugin));
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let circle = meshes.add(Circle::new(20.0));
    let color = Color::linear_rgb(0., 1., 0.);

    commands.spawn((
        Mesh2d(circle),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
    ));
}
