use crate::resources::textures::TextureAssets;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_movement);
    }
}

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((player, mut transform)) = player_query.get_single_mut() {
        let mut move_dir = Vec3::default();

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

        // move_dir will not normalize with a length of zero or close to zero.
        if let Some(normal_move_dir) = move_dir.try_normalize() {
            let length = player.speed * time.delta_seconds();
            transform.translation += normal_move_dir * length;
        }
    }
}

pub fn spawn_player(mut commands: Commands, assets: Res<TextureAssets>) {
    let sprite = TextureAtlasSprite::new(0);
    let bundle = SpriteSheetBundle {
        sprite,
        texture_atlas: assets.player.clone(),
        ..Default::default()
    };

    commands
        .spawn_bundle(bundle)
        .insert(Name::new("Player"))
        .insert(Player { speed: 64. });
}
