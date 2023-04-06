use crate::specs::MergedSpriteSpecs;

/// holds all the output (encoded) sprites.
#[derive(Debug)]
pub struct OutputResult {
    sprites: Vec<EncodedSprite>,
}

impl OutputResult {
    pub fn new() -> Self {
        Self { sprites: vec![] }
    }
    pub fn iter(&self) -> impl Iterator<Item = &EncodedSprite> {
        self.sprites.iter()
    }

    pub fn add(&mut self, sprite: EncodedSprite) {
        self.sprites.push(sprite)
    }
}

/// Holds the data of a sprite encoded in the correct bits per pixel.
#[derive(Debug)]
pub struct EncodedSprite {
    bytes: Vec<u8>,
    width_px: usize,
    height_px: usize,
    name: String,
    bpp: usize,
}

impl EncodedSprite {
    pub fn new(bytes: Vec<u8>, specs: &MergedSpriteSpecs) -> Self {
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
