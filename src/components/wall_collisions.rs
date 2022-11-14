use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use std::collections::{HashMap, HashSet};

use super::physics::WALL_PHYS_LAYER;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct WallCollider;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    pub wall: Wall,
}

/// Represents a wide wall that is 1 tile tall
/// Used to spawn wall collisions
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
struct Plate {
    left: i32,
    right: i32,
}

/// A simple rectangle type representing a wall of any size
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
struct Rect {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

/// Spawns rapier collisions for the walls of a level
///
/// You could just insert a ColliderBundle in to the WallBundle,
/// but this spawns a different collider for EVERY wall tile.
/// This approach leads to bad performance.
///
/// Instead, by flagging the wall tiles and spawning the collisions later,
/// we can minimize the amount of colliding entities.
///
/// The algorithm used here is a nice compromise between simplicity, speed,
/// and a small number of rectangle colliders.
/// In basic terms, it will:
/// 1. consider where the walls are
/// 2. combine wall tiles into flat "plates" in each individual row
/// 3. combine the plates into rectangles across multiple rows wherever possible
/// 4. spawn colliders for each rectangle
pub fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
) {
    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let level_to_wall_locations = parse_level_to_wall(&wall_query, &parent_query);

    if !wall_query.is_empty() {
        level_query.for_each(|(level_entity, level_handle)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let level = levels
                    .get(level_handle)
                    .expect("Level should be loaded by this point");

                let layer_instance = level
                    .level
                    .layer_instances
                    .as_ref()
                    .expect("Level asset should have layers")[0]
                    .clone();

                let width = layer_instance.c_wid;
                let height = layer_instance.c_hei;
                let grid_size = layer_instance.grid_size;

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack = combine_walls_into_plates(width, height, level_walls);

                // combine "plates" into rectangles across multiple rows
                let wall_rects = fill_wall_rects(&mut plate_stack);

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for wall_rect in wall_rects {
                        level
                            .spawn()
                            .insert(WallCollider)
                            .insert(Collider::cuboid(
                                ((wall_rect.right - wall_rect.left + 1) * grid_size) as f32 / 2.,
                                ((wall_rect.top - wall_rect.bottom + 1) * grid_size) as f32 / 2.,
                            ))
                            .insert(RigidBody::Fixed)
                            .insert(Friction {
                                coefficient: 0.1,
                                combine_rule: CoefficientCombineRule::Min,
                            })
                            .insert(Transform::from_xyz(
                                ((wall_rect.left + wall_rect.right + 1) * grid_size) as f32 / 2.,
                                ((wall_rect.bottom + wall_rect.top + 1) * grid_size) as f32 / 2.,
                                0.,
                            ))
                            .insert(CollisionGroups::new(WALL_PHYS_LAYER, Group::all()))
                            .insert(GlobalTransform::default());
                    }
                });
            }
        });
    }
}

fn fill_wall_rects(plate_stack: &mut Vec<Vec<Plate>>) -> Vec<Rect> {
    let mut wall_rects: Vec<Rect> = Vec::new();
    let mut previous_rects: HashMap<Plate, Rect> = HashMap::new();

    // an extra empty row so the algorithm "terminates" the rects that touch the top
    // edge
    plate_stack.push(Vec::new());

    for (y, row) in plate_stack.iter().enumerate() {
        let mut current_rects: HashMap<Plate, Rect> = HashMap::new();
        for plate in row {
            if let Some(previous_rect) = previous_rects.remove(plate) {
                current_rects.insert(
                    *plate,
                    Rect {
                        top: previous_rect.top + 1,
                        ..previous_rect
                    },
                );
            } else {
                current_rects.insert(
                    *plate,
                    Rect {
                        bottom: y as i32,
                        top: y as i32,
                        left: plate.left,
                        right: plate.right,
                    },
                );
            }
        }

        // Any plates that weren't removed above have terminated
        wall_rects.append(&mut previous_rects.values().copied().collect());
        previous_rects = current_rects;
    }

    wall_rects
}

fn parse_level_to_wall(
    wall_query: &Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: &Query<&Parent, Without<Wall>>,
) -> HashMap<Entity, HashSet<GridCoords>> {
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    wall_query.for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_insert_with(HashSet::new)
                .insert(grid_coords);
        }
    });

    level_to_wall_locations
}

fn combine_walls_into_plates(
    width: i32,
    height: i32,
    level_walls: &HashSet<GridCoords>,
) -> Vec<Vec<Plate>> {
    let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

    for y in 0..height {
        let mut row_plates: Vec<Plate> = Vec::new();
        let mut plate_start = None;

        // + 1 to the width so the algorithm "terminates" plates that touch the right
        // edge
        for x in 0..width + 1 {
            match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                (Some(s), false) => {
                    row_plates.push(Plate {
                        left: s,
                        right: x - 1,
                    });
                    plate_start = None;
                }
                (None, true) => plate_start = Some(x),
                _ => (),
            }
        }

        plate_stack.push(row_plates);
    }

    plate_stack
}
