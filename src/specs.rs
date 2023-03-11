use std::{fs, path::PathBuf};

use crate::AppError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Specifications {
    pub tiles: Vec<TileSpec>,
    pub tile_width: usize,
    pub tile_height: usize,
    pub colors_2bpp: Vec<u32>,
    pub colors_1bpp: Vec<u32>,
    pub default_bpp: usize,
}

#[derive(Deserialize, Debug)]
pub struct TileSpec {
    pub name: String,
    pub x: usize,
    pub y: usize,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub bpp: Option<usize>,
}

pub fn load_specs(path: &PathBuf) -> Result<Specifications, AppError> {
    Ok(serde_yaml::from_str(&fs::read_to_string(path)?)?)
}

impl ConcretizedTileSpecs {
    pub fn new(spec: &TileSpec, global: &Specifications) -> ConcretizedTileSpecs {
        let name = spec.name.clone();
        let x_pixels = spec.x * global.tile_width;
        let y_pixels = spec.y * global.tile_height;
        let width_pixels = spec.width.unwrap_or(1) * global.tile_width;
        let height_pixels = spec.height.unwrap_or(1) * global.tile_height;
        let bpp = spec.bpp.unwrap_or(global.default_bpp);
        let colors = if bpp == 1 {
            global.colors_1bpp.clone()
        } else {
            global.colors_2bpp.clone()
        };

        ConcretizedTileSpecs {
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

#[derive(Deserialize, Debug)]
pub struct ConcretizedTileSpecs {
    pub name: String,
    pub x_pixels: usize,
    pub y_pixels: usize,
    pub width_pixels: usize,
    pub height_pixels: usize,
    pub bpp: usize,
    pub colors: Vec<u32>,
}
