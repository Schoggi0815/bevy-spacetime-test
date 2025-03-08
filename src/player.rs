use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player_system);
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct Player;

const SPEED: f32 = 4.;

pub fn move_player_system(
    mut player_query: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut movement = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        movement += Vec3::Y;
    }
    if keys.pressed(KeyCode::KeyS) {
        movement += Vec3::NEG_Y;
    }
    if keys.pressed(KeyCode::KeyD) {
        movement += Vec3::X;
    }
    if keys.pressed(KeyCode::KeyA) {
        movement += Vec3::NEG_X;
    }

    if movement == Vec3::ZERO {
        return;
    }

    movement = movement.normalize() * SPEED;

    for mut player in &mut player_query {
        player.translation += movement;
    }
}
