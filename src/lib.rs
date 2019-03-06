#[macro_use]
extern crate clap;

#[macro_use]
extern crate simple_error;

extern crate byteordered;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

use std::result;

pub mod file;
use file::{FileFormat, FILE_FORMATS};

pub mod pixel;

pub type FileBlob = Vec<u8>;
pub type ImageBlob = Vec<u8>;

pub type RuxResult<T> = result::Result<T, Box<std::error::Error>>;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Dimensions(u32, u32, u32);

#[derive(PartialEq, Debug)]
pub struct Texture {
    pub format: pixel::PixelFormat,
    pub dim: Dimensions,
    pub mip_blobs: Vec<ImageBlob>,
}

pub fn parse(contents: &FileBlob) -> RuxResult<Texture> {
    for variant in &FILE_FORMATS {
        if let Some(codec) = variant.codec() {
            if codec.claim_for_parsing(contents) {
                return codec.parse(contents);
            }
        }
    }
    bail!("Can't identify texture format.");
}

pub fn generate(texture: &Texture, format: FileFormat) -> RuxResult<(FileFormat, FileBlob)> {
    if let Some(codec) = format.codec() {
        return codec.generate(texture, format);
    }
    bail!(format!("Can't generate {:?} textures yet.", format));
}
