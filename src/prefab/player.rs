use std::f32;
use std::sync::Arc;

use failure::Error;
use specs::{Builder, Entity, World};

use embla::assets::image_from_png;
use embla::graphics::TextureImage;
use embla::math::Vec2;

use components::{Player, Sprite, Transform, Velocity};
use prefab::Prefab;

pub enum PlayerPrefab {}

#[derive(Serialize, Deserialize)]
pub struct PlayerConfig {
    pub position: Vec2<f32>,
}

impl Prefab for PlayerPrefab {
    fn create(world: &mut World) -> Result<Entity, Error> {
        Ok(world
            .create_entity()
            .with(Transform::default())
            .with(Velocity::default())
            .with(Sprite {
                texture: TextureImage::new(Arc::new(image_from_png(include_bytes!(
                    "../../assets/ship.png"
                ))?)),
            })
            .with(Player::default())
            .build())
    }
}
