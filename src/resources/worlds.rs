use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(AssetCollection)]
pub struct WorldAssets {
    #[asset(path = "Map.ldtk")]
    pub map: Handle<LdtkAsset>,
}
