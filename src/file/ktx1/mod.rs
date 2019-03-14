use std::io::{Cursor, Read};

use byteordered::{ByteOrdered, Endianness};

use num_traits::*;

use crate::file::{FileCodec, FileFormat};
use crate::{Dimensions, FileBlob, RuxResult, Texture};

use crate::pixel::PixelFormat;

// https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec/#1
//
// Byte[12] identifier
// UInt32 endianness
// UInt32 gl_type
// UInt32 gl_type_size
// UInt32 gl_format
// Uint32 glInternalFormat
// Uint32 glBaseInternalFormat
// UInt32 pixelWidth
// UInt32 pixelHeight
// UInt32 pixelDepth
// UInt32 numberOfArrayElements
// UInt32 numberOfFaces
// UInt32 numberOfMipmapLevels
// UInt32 bytesOfKeyValueData

// for each keyValuePair that fits in bytesOfKeyValueData
//     UInt32   keyAndValueByteSize
//     Byte     keyAndValue[keyAndValueByteSize]
//     Byte     valuePadding[3 - ((keyAndValueByteSize + 3) % 4)]
// end

// for each mipmap_level in numberOfMipmapLevels1
//     UInt32 imageSize;
//     for each array_element in numberOfArrayElements2
//        for each face in numberOfFaces3
//            for each z_slice in pixelDepth2
//                for each row or row_of_blocks in pixelHeight2
//                    for each pixel or block_of_pixels in pixelWidth
//                        Byte data[format-specific-number-of-bytes]4
//                    end
//                end
//            end
//            Byte cubePadding[0-3]
//        end
//     end
//     Byte mipPadding[0-3]
// end

const KTX1_MAGIC: [u8; 12] = [
    0xAB, 0x4B, 0x54, 0x58, 0x20, 0x31, 0x31, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];

const KTX1_BIG_ENDIAN: [u8; 4] = [0x04, 0x03, 0x02, 0x01];
const KTX1_LITTLE_ENDIAN: [u8; 4] = [0x01, 0x02, 0x03, 0x04];

const HEADER_SIZE: usize = 64;

pub struct Ktx1Codec;

impl FileCodec for Ktx1Codec {
    fn claim_for_parsing(&self, contents: &FileBlob) -> bool {
        contents.len() >= 12 && contents[0..12] == KTX1_MAGIC
    }
    fn parse(&self, contents: &FileBlob) -> RuxResult<Texture> {
        // this is an invariant here
        assert_eq!(contents[0..12], KTX1_MAGIC);

        if contents.len() < HEADER_SIZE {
            bail!(format!(
                "KTX1: Invalid file; header truncated at {} bytes",
                contents.len()
            ));
        }

        let mut cursor = Cursor::new(&contents[12..]);
        let mut endian_buf = [0x00; 4];
        cursor.read_exact(&mut endian_buf).unwrap(); // todo

        let mut reader = ByteOrdered::runtime(
            cursor,
            match endian_buf {
                KTX1_BIG_ENDIAN => Endianness::Big,
                KTX1_LITTLE_ENDIAN => Endianness::Little,
                _ => bail!("KTX1: Field 'endianness' is invalid"),
            },
        );

        let gl_type = reader.read_u32()?;
        // if gl_type != 0 {
        //     bail!("KTX1: Field 'gl_type' must be 0. (This tool temporarily supports only compressed textures.)");
        // }
        let gl_type_size = reader.read_u32()?;
        // if gl_type_size != 1 {
        //     bail!("KTX1: Field 'gl_type_size' must be 1. (This tool temporarily supports only compressed textures.)");
        // }
        let gl_format = reader.read_u32()?;
        // if gl_format != 0 {
        //     bail!("KTX1: Field 'gl_format' must be 0. (This tool temporarily supports only compressed textures.)");
        // }

        let gl_internal_format_num = reader.read_u32()?;

        let ktx_lookup = crate::pixel::gl::GlFormat::from_u32(gl_internal_format_num)
            .and_then(|format| PixelFormat::for_gl_format(format));

        let format = if let Some(format) = ktx_lookup {
            format
        } else {
            bail!(format!(
                "KTX1: Field 'gl_internal_format' references unknown format: {:#4X}",
                gl_internal_format_num
            ));
        };

        // let gl_base_internal_format = reader.read_u32()?;
        // if gl_base_internal_format != ktx_mapping.base_internal_format.to_u32().unwrap() {
        //     println!(
        //         "KTX1: Warning: Field 'gl_base_internal_format' should be {:#4X}, but is {:#4X}.",
        //         ktx_mapping.base_internal_format.to_u32().unwrap(),
        //         gl_base_internal_format,
        //     );
        // }
        let pixel_width = reader.read_u32()?;
        let pixel_height = reader.read_u32()?;
        let pixel_depth = reader.read_u32()?;
        let number_of_array_elements = reader.read_u32()?;
        let number_of_faces = reader.read_u32()?;
        let number_of_mipmap_levels = reader.read_u32()?;
        let bytes_of_key_value_data = reader.read_u32()?;

        Ok(Texture {
            format: format,
            dim: Dimensions(pixel_width, pixel_height, pixel_depth),
            mip_blobs: vec![],
        })
    }
    fn generate(&self, texture: &Texture, format: FileFormat) -> RuxResult<(FileFormat, FileBlob)> {
        bail!("KTX1 generation not yet implemented");
    }
}
