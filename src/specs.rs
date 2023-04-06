use std::{fs, path::PathBuf};

use crate::AppError;
use serde::Deserialize;

/// Top level specifications file.
/// Gets deserialized from yaml by serde.
#[derive(Deserialize, Debug)]
pub struct Specifications {
    pub sprites: Vec<SpriteSpecs>,
    pub sprite_width: usize,
    pub sprite_height: usize,
    pub colors_2bpp: Vec<u32>,
    pub colors_1bpp: Vec<u32>,
    pub default_bpp: usize,
}

/// Inner definition for each sprite
#[derive(Deserialize, Debug)]
pub struct SpriteSpecs {
    pub name: String,
    pub x: usize,
    pub y: usize,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub bpp: Option<usize>,
}

/// Loads the specifications from the passed file.
pub fn load_specs(path: &PathBuf) -> Result<Specifications, AppError> {
    Ok(serde_yaml::from_str(&fs::read_to_string(path)?)?)
}

/// The SpriteSpecs object has many defaultable fields and is missing some data from the toplevel file options.
/// This object holds all the needed data for each Sprite, with all the default values applied.
/// It also converts all the grid-based measurements in pixels for ease of use later.
#[derive(Deserialize, Debug)]
pub struct MergedSpriteSpecs {
    pub name: String,
    pub x_pixels: usize,
    pub y_pixels: usize,
    pub width_pixels: usize,
    pub height_pixels: usize,
    pub bpp: usize,
    pub colors: Vec<u32>,
}

impl MergedSpriteSpecs {
    /// Generates the concrete/merged specs from all the available specifications.
    pub fn new(spec: &SpriteSpecs, global: &Specifications) -> MergedSpriteSpecs {
        let name = spec.name.clone();
        let x_pixels = spec.x * global.sprite_width;
        let y_pixels = spec.y * global.sprite_height;
        let width_pixels = spec.width.unwrap_or(1) * global.sprite_width;
        let height_pixels = spec.height.unwrap_or(1) * global.sprite_height;
        let bpp = spec.bpp.unwrap_or(global.default_bpp);
        let colors = if bpp == 1 {
            global.colors_1bpp.clone()
        } else {
            global.colors_2bpp.clone()
        };

        MergedSpriteSpecs {
            name,
            x_pixels,
            y_pixels,
            width_pixels,
            height_pixels,
            bpp,
            colors,
        }
    }
}
