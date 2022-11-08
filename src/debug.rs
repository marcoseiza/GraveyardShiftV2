use bevy::prelude::*;
use bevy_inspector_egui::{prelude::*, widgets::*};
use bevy_pixel_camera::PixelProjection;

use crate::{mutant::Mutant, player::Player};

#[derive(Inspectable, Default)]
pub struct Entities {
    #[inspectable(collapse, label = "Player")]
    player: InspectorQuerySingle<Entity, With<Player>>,
    #[inspectable(collapse, label = "Mutants")]
    mutants: InspectorQuery<Entity, With<Mutant>>,
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .add_plugin(InspectorPlugin::<Entities>::new())
                .register_type::<PixelProjection>()
                .register_inspectable::<Player>();
        }
    }
}
