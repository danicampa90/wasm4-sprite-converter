use crate::errors::AppError;
use crate::specs::MergedSpriteSpecs;
use crate::EncodedSprite;
use image::RgbImage;

pub fn encode_sprite(
    sprite_spec: &MergedSpriteSpecs,
    image: &RgbImage,
) -> Result<EncodedSprite, AppError> {
    // a list of palette indexes corresponding to each pixel.
    let mut pixel_palette_indexes = vec![];

    // scan image and get the indexes into the palette
    for y in 0..sprite_spec.height_pixels {
        for x in 0..sprite_spec.width_pixels {
            let x = (sprite_spec.x_pixels + x) as u32;
            let y = (sprite_spec.y_pixels + y) as u32;
            let pix = image.get_pixel(x, y);
            let combined = (pix[0] as u32) << 16 | (pix[1] as u32) << 8 | (pix[2] as u32) << 0;
            let color_index = sprite_spec
                .colors
                .iter()
                .position(|element| *element == combined);

            match color_index {
                None => {
                    println!("Color info for not found color: {:#08x}", combined);
                    return Err(AppError::CannotFindColor {
                        x,
                        y,
                        color: combined,
                    });
                }
                Some(index) => {
                    pixel_palette_indexes.push(index);
                }
            }
        }
    }

    // encode the paletted image into packed bytes.
    let mut resulting_bytes = vec![];

    // each "big step" keeps track of the index into pixel_palette_indexes, so that at the beginning of each step it's aligned with a byte.
    // this guarantees that for each big step we can write out a byte to the output.
    for big_step_idx in (0..pixel_palette_indexes.len()).step_by(8 / sprite_spec.bpp) {
        let mut byte: u8 = 0;
        // for each "intrabyte step" we bit-manipulate the byte that will be written in the outer loop
        for intrabyte_idx in 0..(8 / sprite_spec.bpp) {
            byte = byte << sprite_spec.bpp;
            let pixel = pixel_palette_indexes
                .get(intrabyte_idx + big_step_idx)
                .cloned();

            // if we run out of pixels to write we still need to pad this last byte with zeroes.
            byte |= pixel.unwrap_or(0) as u8;
            //println!("{:08b}: {:?}",byte, pixel )
        }
        //println!("{:08b}", byte);
        resulting_bytes.push(byte);
    }

    // print stats
    println!("- {}: {} bytes", sprite_spec.name, resulting_bytes.len());

    // result
    Ok(EncodedSprite::new(resulting_bytes, sprite_spec))
}
