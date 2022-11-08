use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_pixel_camera::PixelCameraBundle;

#[derive(Component)]
pub struct LerpSpeed(pub f32);

impl Default for LerpSpeed {
    fn default() -> Self {
        Self(1.)
    }
}

pub fn spawn_camera(mut commands: Commands) {
    let camera = PixelCameraBundle::from_zoom(2);
    commands
        .spawn_bundle(camera)
        .insert(LerpSpeed(3.0))
        .insert(Name::new("Pixel Camera"));
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default, Inspectable)]
pub struct CameraAnchor(pub i32);

#[allow(clippy::type_complexity)]
pub fn camera_follow_anchor(
    anchor_query: Query<(&Transform, &CameraAnchor)>,
    mut camera_query: Query<(&mut Transform, &LerpSpeed), (Without<CameraAnchor>, With<Camera>)>,
    time: Res<Time>,
) {
    if let Some((anchor_transform, _)) = anchor_query
        .iter()
        .max_by(|(_, &anchor_a), (_, &anchor_b)| anchor_a.0.cmp(&anchor_b.0))
    {
        let (mut camera_transform, lerp_speed) = camera_query.single_mut();

        camera_transform.translation = camera_transform.translation.lerp(
            anchor_transform.translation,
            lerp_speed.0 * time.delta_seconds(),
        )
    }
}
