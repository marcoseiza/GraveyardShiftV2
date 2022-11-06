use bevy::prelude::*;
use bevy_pixel_camera::PixelCameraBundle;

pub fn spawn_camera(mut commands: Commands) {
    let camera = PixelCameraBundle::from_zoom(2);
    commands
        .spawn_bundle(camera)
        .insert(Name::new("Pixel Camera"));
}
