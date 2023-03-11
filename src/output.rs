use crate::specs::ConcretizedTileSpecs;

#[derive(Debug)]
pub struct OutputResult {
    tiles: Vec<OutputTileData>,
}

#[derive(Debug)]
pub struct OutputTileData {
    bytes: Vec<u8>,
    width_px: usize,
    height_px: usize,
    name: String,
    bpp: usize,
}

impl OutputResult {
    pub fn new() -> Self {
        Self { tiles: vec![] }
    }
    pub fn iter(&self) -> impl Iterator<Item = &OutputTileData> {
        self.tiles.iter()
    }

    pub fn add(&mut self, tile: OutputTileData) {
        self.tiles.push(tile)
    }
}

impl OutputTileData {
    pub fn new(bytes: Vec<u8>, specs: &ConcretizedTileSpecs) -> Self {
        Self {
            bytes,
            name: specs.name.clone(),
            bpp: specs.bpp,
            width_px: specs.width_pixels,
            height_px: specs.height_pixels,
        }
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn bpp(&self) -> usize {
        self.bpp
    }
    pub fn width_pixels(&self) -> usize {
        self.width_px
    }
    pub fn height_pixels(&self) -> usize {
        self.height_px
    }
}
