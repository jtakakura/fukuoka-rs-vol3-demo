use quicksilver::{combinators::join_all, graphics::Image, lifecycle::Asset, sound::Sound, Future};
use std::collections::HashMap;

pub const IMAGES: &[&str] = &["bullet_b.png", "bullet_w.png"];
pub const SOUNDS: &[&str] = &[];
pub const FONTS: &[&str] = &[];

pub struct Assets {
    pub bullet_b: Image,
    pub bullet_w: Image,
}

impl Assets {
    pub fn new() -> Asset<Assets> {
        let image_loader =
            join_all(IMAGES.iter().cloned().map(Image::load)).map(|loaded: Vec<Image>| {
                IMAGES
                    .iter()
                    .cloned()
                    .zip(loaded)
                    .collect::<HashMap<&'static str, Image>>()
            });

        let sound_loader =
            join_all(SOUNDS.iter().cloned().map(Sound::load)).map(|loaded: Vec<Sound>| {
                SOUNDS
                    .iter()
                    .cloned()
                    .zip(loaded)
                    .collect::<HashMap<&'static str, Sound>>()
            });

        let loaders = image_loader.join(sound_loader);

        Asset::new(loaders.map(|(images, sounds)| Assets {
            bullet_b: images["bullet_b.png"].clone(),
            bullet_w: images["bullet_w.png"].clone(),
        }))
    }
}
