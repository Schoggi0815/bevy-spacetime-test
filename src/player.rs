use bevy::prelude::*;
use bevy_hookup_core::{
    owner_component::Owner, sendable_component::SendableComponent, shared::Shared,
};
use serde::{Deserialize, Serialize};

use crate::sendables::Sendables;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_player_system,
                move_player_smoothed,
                update_player_smoothing,
                add_other_player,
            ),
        )
        .add_systems(FixedUpdate, update_player_sync);
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct Player {
    pub velocity: Vec3,
    pub movement_speed: f32,
}

#[derive(Component)]
pub struct PreviousPosition {
    pub previous_position: Vec3,
}

#[derive(Component, Default)]
struct PlayerSmoothing {
    lerp_time: f32,
    start_pos: Vec3,
    end_pos: Vec3,
}

#[derive(Default, Clone, Reflect, Serialize, Deserialize)]
pub struct PlayerSync {
    pub velocity: Vec3,
    pub position: Vec3,
}

impl SendableComponent<Sendables> for PlayerSync {
    fn to_sendable(&self) -> Sendables {
        Sendables::PlayerSync(self.clone())
    }

    fn from_sendable(sendable: Sendables) -> Option<Self> {
        match sendable {
            Sendables::PlayerSync(player_sync) => Some(player_sync),
        }
    }
}

const SPEED: f32 = 400.;

fn move_player_system(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
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
        for (_, mut player) in &mut player_query {
            if player.velocity == Vec3::ZERO {
                continue;
            }
            player.velocity = movement;
        }

        return;
    }

    movement = movement.normalize() * SPEED;

    let adjusted_movement = movement * time.delta_secs();

    for (mut transform, mut player) in &mut player_query {
        player.velocity = movement * player.movement_speed;
        transform.translation += adjusted_movement * player.movement_speed;
    }
}

fn update_player_sync(player_syncs: Query<(Ref<Player>, Ref<Transform>, &mut Owner<PlayerSync>)>) {
    for (player, transform, mut player_sync) in player_syncs {
        if !player.is_changed() && !transform.is_changed() {
            continue;
        }

        player_sync.velocity = player.velocity;
        player_sync.position = transform.translation;
    }
}

fn update_player_smoothing(
    other_players: Query<
        (&Shared<PlayerSync>, &mut PlayerSmoothing, &Transform),
        Changed<Shared<PlayerSync>>,
    >,
) {
    for (player_sync, mut player_smoothing, transfrom) in other_players {
        player_smoothing.lerp_time = 0.;
        player_smoothing.start_pos = transfrom.translation;
        player_smoothing.end_pos = player_sync.position + player_sync.velocity * 0.2;
    }
}

fn move_player_smoothed(
    player_smoothings: Query<(&mut PlayerSmoothing, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut player_smoothing, mut transform) in player_smoothings {
        player_smoothing.lerp_time += time.delta_secs() * 5.;
        let new_pos = player_smoothing
            .start_pos
            .lerp(player_smoothing.end_pos, player_smoothing.lerp_time.min(1.));
        transform.translation = new_pos;
    }
}

fn add_other_player(
    player_syncs: Query<Entity, Added<Shared<PlayerSync>>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in player_syncs {
        let circle = meshes.add(Circle::new(20.0));
        let color = Color::linear_rgb(1., 0., 0.);

        commands.entity(entity).insert((
            Mesh2d(circle),
            MeshMaterial2d(materials.add(color)),
            PlayerSmoothing::default(),
            Transform::default(),
        ));
    }
}
