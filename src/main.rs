use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_asset_loader::prelude::*;
use bevy_pixel_camera::PixelCameraPlugin;
use debug::*;
use game_state::GameState;
use graphics::camera::spawn_camera;
use iyes_loopless::prelude::*;
use player::*;
use resources::textures::*;

mod debug;
mod game_state;
mod graphics;
mod player;
mod resources;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            width: 1280.0,
            height: 720.0,
            title: "Graveyard Shift".into(),
            present_mode: bevy::window::PresentMode::AutoVsync,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_loopless_state(GameState::AssetLoading)
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                // https://github.com/NiklasEi/bevy_asset_loader/issues/54
                .continue_to_state(GameState::Playing)
                .with_collection::<TextureAssets>(),
        )
        // ==== Setup ====
        .add_enter_system(GameState::Playing, spawn_camera)
        .add_enter_system(GameState::Playing, spawn_player)
        // ===============
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(DebugPlugin)
        .add_plugin(PixelCameraPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
