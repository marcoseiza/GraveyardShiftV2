use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{components::physics::ColliderBundle, graphics::camera::CameraAnchor};

#[derive(Component, Default, Inspectable)]
pub struct Mutant;

#[derive(Bundle, Default, LdtkEntity)]
pub struct MutantBundle {
    #[with(mutant_collider)]
    #[bundle]
    collider: ColliderBundle,
    mutant: Mutant,
    #[sprite_sheet_bundle("mutant.png", 32.0, 32.0, 4, 1, 0.0, 0.0, 0)]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    #[with(mutant_camera_anchor)]
    camera_anchor: CameraAnchor,
}

fn mutant_camera_anchor(_: EntityInstance) -> CameraAnchor {
    CameraAnchor(0)
}

fn mutant_collider(_: EntityInstance) -> ColliderBundle {
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
