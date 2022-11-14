use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub damping: Damping,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub friction: Friction,
    pub restitution: Restitution,
    pub mass_properties: ColliderMassProperties,
    pub force: ExternalForce,
    pub solver_groups: SolverGroups,
    pub collision_groups: CollisionGroups,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> ColliderBundle {
        #[allow(clippy::match_single_binding)]
        match entity_instance.identifier {
            _ => ColliderBundle::default(),
        }
    }
}

pub const WALL_PHYS_LAYER: Group = Group::GROUP_1;
pub const PLAYER_PHYS_LAYER: Group = Group::GROUP_2;
pub const MUTANT_PHYS_LAYER: Group = Group::GROUP_3;
pub const SOUND_PHYS_LAYER: Group = Group::GROUP_10;
