use std::f32::consts::PI;

use crate::components::physics::*;
use crate::components::wall_collisions::WallCollider;
use crate::graphics::camera::CameraAnchor;
use bevy::prelude::shape::Circle;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::na::ComplexField;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_movement)
            .add_system(player_sound)
            .add_system(kill_old_sounds)
            .add_system(wall_sound_collisions);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component, Inspectable)]
pub struct Player;

#[derive(Component, Default, Clone, Inspectable)]
pub struct MovementForce(pub f32);

fn player_movement(
    mut player_query: Query<(&MovementForce, &mut ExternalForce), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if let Ok((MovementForce(mvt_force), mut ext_force)) = player_query.get_single_mut() {
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
            ext_force.force = move_dir * (*mvt_force);
        } else {
            ext_force.force = Vec2::splat(0.);
        }
    }
}

#[derive(Component)]
struct SoundPoint;

#[derive(Component)]
struct SoundLifetime(Timer);

fn kill_old_sounds(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SoundLifetime)>,
    time: Res<Time>,
) {
    for (entity, mut sound_lifetime) in query.iter_mut() {
        // timers gotta be ticked, to work
        sound_lifetime.0.tick(time.delta());

        // if it finished, despawn the bomb
        if sound_lifetime.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn make_sound_dots(
    number_of_dots: i8,
    commands: &mut Commands,
    transform: &Transform,
    mesh: &Mesh2dHandle,
    material: &Handle<ColorMaterial>,
) {
    let mut make_sound_dot = |velocity: Vec2| {
        commands
            .spawn_bundle(ColorMesh2dBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: *transform,
                ..Default::default()
            })
            .insert(SoundPoint)
            .insert_bundle(ColliderBundle {
                collider: Collider::ball(1.0),
                rigid_body: RigidBody::KinematicVelocityBased,
                collision_groups: CollisionGroups::new(SOUND_PHYS_LAYER, WALL_PHYS_LAYER),
                ..Default::default()
            })
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Velocity {
                linvel: velocity,
                ..Default::default()
            })
            .insert(Ccd::enabled())
            .insert(Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Max,
            })
            .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC)
            .insert(Name::new("Sound Dot"))
            .insert(SoundLifetime(Timer::from_seconds(2., false)));
    };

    for i in 0i8..number_of_dots {
        let angle = f32::from(i) * 2. * PI / number_of_dots as f32;
        make_sound_dot(Vec2::new(angle.cos(), angle.sin()) * 50.);
    }
}

fn player_sound(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if let Ok(transform) = player_query.get_single() {
            let circle = Circle::new(1.0);
            let handle = meshes.add(Mesh::from(circle));
            let material = ColorMaterial::from(Color::WHITE);
            let material_handle = materials.add(material);

            make_sound_dots(
                64,
                &mut commands,
                transform,
                &handle.into(),
                &material_handle,
            );
        }
    }
}

fn wall_sound_collisions(
    rapier_context: Res<RapierContext>,
    mut sound_query: Query<(Entity, &mut Velocity), With<SoundPoint>>,
    wall_query: Query<Entity, With<WallCollider>>,
) {
    for (entity, mut sound_velocity) in sound_query.iter_mut() {
        for contact in rapier_context.contacts_with(entity) {
            if contact.has_any_active_contacts() {
                let other_collider = if contact.collider1() == entity {
                    contact.collider2()
                } else {
                    contact.collider1()
                };

                if wall_query.contains(other_collider) {
                    if let Some(manifold) = contact.manifold(0) {
                        let normal = manifold.normal();
                        if normal.x.abs() > 0. {
                            sound_velocity.linvel.x *= -1.;
                        }
                        if normal.y.abs() > 0. {
                            sound_velocity.linvel.y *= -1.;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Bundle, Default, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[with(player_movement_force)]
    mvt_force: MovementForce,
    #[with(player_collider)]
    #[bundle]
    collider: ColliderBundle,
    #[with(player_camera_anchor)]
    camera_anchor: CameraAnchor,
    #[sprite_sheet_bundle("player.png", 32.0, 32.0, 8, 1, 0.0, 0.0, 0)]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

fn player_movement_force(_: EntityInstance) -> MovementForce {
    MovementForce(15.0)
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
            linear_damping: 13.0,
            ..Default::default()
        },
        collision_groups: CollisionGroups::new(PLAYER_PHYS_LAYER, Group::all()),
        ..Default::default()
    }
}
