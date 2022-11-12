use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use bevy_pixel_camera::*;
use bevy_rapier2d::prelude::*;
use components::wall_collisions::*;
use debug::*;
use game_state::GameState;
use graphics::camera::*;
use iyes_loopless::prelude::*;
use mutant::MutantBundle;
use player::*;
use resources::*;

mod components;
mod debug;
mod game_state;
mod graphics;
mod mutant;
mod player;
mod resources;
mod utils;

const LEVEL_ONE_ID: &str = "01a63d70-5110-11ed-a5d6-d713966358e6";

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
                .continue_to_state(GameState::Playing)
                .with_collection::<WorldAssets>(),
        )
        // ===============
        .insert_resource(LevelSelection::Iid(LEVEL_ONE_ID.into()))
        .add_enter_system(GameState::Playing, spawn_camera)
        .add_enter_system(GameState::Playing, spawn_world)
        // ===============
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(DebugPlugin)
        .add_plugin(PixelCameraPlugin)
        .add_system(spawn_wall_collision)
        .add_system_to_stage(CoreStage::PostUpdate, camera_follow_anchor)
        .add_plugin(LdtkPlugin)
        .insert_resource(LdtkSettings {
            int_grid_rendering: IntGridRendering::Invisible,
            ..Default::default()
        })
        .add_plugin(PlayerPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_inspectable::<CameraAnchor>()
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<MutantBundle>("Mutant")
        .run();
}

fn spawn_world(mut commands: Commands, assets: Res<WorldAssets>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: assets.map.clone(),
        ..Default::default()
    });
}
