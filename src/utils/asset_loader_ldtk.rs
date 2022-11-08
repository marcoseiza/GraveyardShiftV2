use bevy::prelude::*;

use crate::resources::TextureAssets;

pub trait LdtkTextureAssetLoader: 'static {
    fn get_asset(&self, assets: &Res<TextureAssets>) -> Handle<TextureAtlas>;
}

// impl_trait_query!(LdtkTextureAssetLoader);
