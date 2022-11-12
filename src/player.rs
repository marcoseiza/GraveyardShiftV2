use crate::components::physics::*;
use crate::graphics::camera::CameraAnchor;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_movement);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component, Inspectable)]
pub struct Player;

const PLAYER_TARGET_SPEED: f32 = 100.0;

fn player_movement(
    mut player_query: Query<(Entity, &Velocity, &mut ExternalForce), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if let Ok((_player, velocity, mut ext_force)) = player_query.get_single_mut() {
        let mut move_dir = Vec2::default();

        if keyboard.pressed(KeyCode::W) {
            move_dir.y += 1.;
        }
        if keyboard.pressed(KeyCode::S) {
            move_dir.y -= 1.;
        }
        if keyboard.pressed(KeyCode::D) {
            move_dir.x += 1.;
        }
        if keyboard.pressed(KeyCode::A) {
            move_dir.x -= 1.;
        }

        // If move_dir is not a vector of length 0.0.
        if move_dir.try_normalize().is_some() {
            ext_force.force = (move_dir * PLAYER_TARGET_SPEED) - velocity.linvel;
        } else {
            ext_force.force = Vec2::splat(0.);
        }
    }
}

#[derive(Bundle, Default, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[with(player_collider)]
    #[bundle]
    collider: ColliderBundle,
    #[with(player_camera_anchor)]
    camera_anchor: CameraAnchor,
    #[sprite_sheet_bundle("player.png", 32.0, 32.0, 8, 1, 0.0, 0.0, 0)]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

fn player_camera_anchor(_: EntityInstance) -> CameraAnchor {
    CameraAnchor(1)
}

fn player_collider(_: EntityInstance) -> ColliderBundle {
    ColliderBundle {
        collider: Collider::capsule(Vec2::new(0., -4.), Vec2::new(0., -12.), 5.),
        rigid_body: RigidBody::Dynamic,
        rotation_constraints: LockedAxes::ROTATION_LOCKED,
        damping: Damping {
            linear_damping: 10.0,
            ..Default::default()
        },
        ..Default::default()
    }
}
