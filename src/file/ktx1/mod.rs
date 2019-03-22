use std::cmp;
use std::io::{Cursor, Read};

use byteordered::{ByteOrdered, Endianness};

use num_traits::*;

use crate::file::{FileCodec, FileFormat};
use crate::{Dimensions, FileBlob, RuxResult, Texture};

use crate::pixel::gl::{gl_upgrade_old_formats, GlBaseFormat, GlDataType, GlFormat};
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
        cursor.read_exact(&mut endian_buf)?;

        let mut reader = ByteOrdered::runtime(
            cursor,
            match endian_buf {
                KTX1_BIG_ENDIAN => Endianness::Big,
                KTX1_LITTLE_ENDIAN => Endianness::Little,
                _ => bail!("KTX1: Field 'endianness' is invalid"),
            },
        );

        let gl_type = reader.read_u32()?;
        let _gl_type_size = reader.read_u32()?; // TODO: validate
        let gl_format = reader.read_u32()?;

        let gl_internal_format_num = reader.read_u32()?;
        let gl_internal_format =
            GlFormat::from_u32(gl_internal_format_num).map(gl_upgrade_old_formats);

        let format_matches =
            gl_internal_format.map_or(vec![], |format| PixelFormat::for_gl_format(format));

        if format_matches.is_empty() {
            bail!(format!(
                "KTX1: Field 'gl_internal_format' references unknown format: {:#4X}",
                gl_internal_format_num
            ));
        }

        let format = {
            if format_matches.iter().any(|f| f.is_compressed()) {
                if !format_matches.iter().all(|f| f.is_compressed()) {
                    panic!(
                        "KTX1: INTERNAL ERROR: Field 'gl_internal_format' has both compressed and uncompressed matches: {:#4X}",
                        gl_internal_format_num
                    );
                }
                if format_matches.len() > 1 {
                    panic!(
                        "KTX1: INTERNAL ERROR: Field 'gl_internal_format' has multiple compressed matches: {:#4X}",
                        gl_internal_format_num
                    );
                }
                format_matches[0]
            } else {
                let good_matches: Vec<&PixelFormat> = format_matches
                    .iter()
                    .filter(|m| {
                        let type_match = crate::pixel::gl::to_gl(&m.comp_layout, &m.comp_content);
                        if let Some((matched_format, matched_type)) = type_match {
                            matched_format.to_u32().unwrap() == gl_format
                                && matched_type.to_u32().unwrap() == gl_type
                        } else {
                            false
                        }
                    })
                    .cloned()
                    .collect();

                let good_matches = if good_matches.is_empty() {
                    println!("Warning: ignoring incompatible fields gl_type/gl_format.");
                    format_matches
                } else {
                    good_matches
                };

                if good_matches.len() > 1 {
                    // TODO: figure out how to decide whether to look for an sRGB variant
                    println!(
                        "KTX1: (TODO) Field 'gl_internal_format' is ambiguous: {:#4X}",
                        gl_internal_format_num
                    );
                }
                good_matches[0]
            }
        };

        let pixel_width = reader.read_u32()?;
        let pixel_height = reader.read_u32()?;
        let pixel_depth = reader.read_u32()?;
        let number_of_array_elements = reader.read_u32()?;
        let number_of_faces = reader.read_u32()?;
        let number_of_mipmap_levels = cmp::max(1, reader.read_u32()?);
        let mut bytes_of_key_value_data = reader.read_u32()? as isize;

        // TODO: actually do something with the key-value pairs
        while bytes_of_key_value_data > 0 {
            let kv_size = reader.read_u32()?;
            let mut kv_buf = vec![0x00; kv_size as usize];
            reader.read_exact(&mut kv_buf)?;
            bytes_of_key_value_data -= 4 + kv_size;
            while (bytes_of_key_value_data % 4) != 0 {
                reader.read_u8()?;
            }
        }

        let mut mip_blobs = vec![];
        // TODO: properly handle imageSize & cubePadding in the cubemap case
        for _mip_level in 0..number_of_mipmap_levels {
            let image_size = reader.read_u32()?;
            // TODO: validate image_size against dimensions & pixel format
            let mut image_buf = vec![0x00; image_size as usize];
            reader.read_exact(&mut image_buf)?;
            mip_blobs.push(image_buf);
        }

        Ok(Texture {
            format: *format,
            pixel_dim: Dimensions(pixel_width, pixel_height, pixel_depth),
            array_size: number_of_array_elements,
            face_count: number_of_faces,
            mip_blobs,
        })
    }
    fn generate(
        &self,
        _texture: &Texture,
        _format: FileFormat,
    ) -> RuxResult<(FileFormat, FileBlob)> {
        bail!("KTX1 generation not yet implemented");
    }
}

fn _foo() {
    let _tests = vec![
        (
            GlFormat::DEPTH_COMPONENT16,
            GlBaseFormat::DEPTH_COMPONENT,
            GlDataType::UNSIGNED_SHORT,
        ),
        (
            GlFormat::DEPTH_COMPONENT24,
            GlBaseFormat::DEPTH_COMPONENT,
            GlDataType::UNSIGNED_INT,
        ),
        (
            GlFormat::DEPTH_COMPONENT32F,
            GlBaseFormat::DEPTH_COMPONENT,
            GlDataType::FLOAT,
        ),
        (
            GlFormat::DEPTH24_STENCIL8,
            GlBaseFormat::DEPTH_STENCIL,
            GlDataType::UNSIGNED_INT_24_8,
        ),
        (
            GlFormat::DEPTH32F_STENCIL8,
            GlBaseFormat::DEPTH_STENCIL,
            GlDataType::FLOAT_32_UNSIGNED_INT_24_8_REV,
        ),
        (
            GlFormat::R11F_G11F_B10F,
            GlBaseFormat::RGB,
            GlDataType::UNSIGNED_INT_10F_11F_11F_REV,
        ),
        (GlFormat::R16, GlBaseFormat::RED, GlDataType::UNSIGNED_SHORT),
        (GlFormat::R16_SNORM, GlBaseFormat::RED, GlDataType::SHORT),
        (GlFormat::R16F, GlBaseFormat::RED, GlDataType::HALF_FLOAT),
        (GlFormat::R16I, GlBaseFormat::RED_INTEGER, GlDataType::SHORT),
        (
            GlFormat::R16UI,
            GlBaseFormat::RED_INTEGER,
            GlDataType::UNSIGNED_SHORT,
        ),
        (GlFormat::R32F, GlBaseFormat::RED, GlDataType::FLOAT),
        (GlFormat::R32I, GlBaseFormat::RED_INTEGER, GlDataType::INT),
        (
            GlFormat::R32UI,
            GlBaseFormat::RED_INTEGER,
            GlDataType::UNSIGNED_INT,
        ),
        (GlFormat::R8, GlBaseFormat::RED, GlDataType::UNSIGNED_BYTE),
        (GlFormat::R8_SNORM, GlBaseFormat::RED, GlDataType::BYTE),
        (GlFormat::R8I, GlBaseFormat::RED_INTEGER, GlDataType::BYTE),
        (
            GlFormat::R8UI,
            GlBaseFormat::RED_INTEGER,
            GlDataType::UNSIGNED_BYTE,
        ),
        (GlFormat::RG16, GlBaseFormat::RG, GlDataType::UNSIGNED_SHORT),
        (GlFormat::RG16_SNORM, GlBaseFormat::RG, GlDataType::SHORT),
        (GlFormat::RG16F, GlBaseFormat::RG, GlDataType::HALF_FLOAT),
        (GlFormat::RG16I, GlBaseFormat::RG_INTEGER, GlDataType::SHORT),
        (
            GlFormat::RG16UI,
            GlBaseFormat::RG_INTEGER,
            GlDataType::UNSIGNED_SHORT,
        ),
        (GlFormat::RG32F, GlBaseFormat::RG, GlDataType::FLOAT),
        (GlFormat::RG32I, GlBaseFormat::RG_INTEGER, GlDataType::INT),
        (
            GlFormat::RG32UI,
            GlBaseFormat::RG_INTEGER,
            GlDataType::UNSIGNED_INT,
        ),
        (GlFormat::RG8, GlBaseFormat::RG, GlDataType::UNSIGNED_BYTE),
        (GlFormat::RG8_SNORM, GlBaseFormat::RG, GlDataType::BYTE),
        (GlFormat::RG8I, GlBaseFormat::RG_INTEGER, GlDataType::BYTE),
        (
            GlFormat::RG8UI,
            GlBaseFormat::RG_INTEGER,
            GlDataType::UNSIGNED_BYTE,
        ),
        (
            GlFormat::RGB10_A2,
            GlBaseFormat::BGRA,
            GlDataType::UNSIGNED_INT_2_10_10_10_REV,
        ),
        (
            GlFormat::RGB10_A2,
            GlBaseFormat::RGBA,
            GlDataType::UNSIGNED_INT_2_10_10_10_REV,
        ),
        (
            GlFormat::RGB10_A2UI,
            GlBaseFormat::BGRA_INTEGER,
            GlDataType::UNSIGNED_INT_2_10_10_10_REV,
        ),
        (
            GlFormat::RGB10_A2UI,
            GlBaseFormat::RGBA_INTEGER,
            GlDataType::UNSIGNED_INT_2_10_10_10_REV,
        ),
        (
            GlFormat::RGB16,
            GlBaseFormat::RGB,
            GlDataType::UNSIGNED_SHORT,
        ),
        (GlFormat::RGB16_SNORM, GlBaseFormat::RGB, GlDataType::SHORT),
        (GlFormat::RGB16F, GlBaseFormat::RGB, GlDataType::HALF_FLOAT),
        (
            GlFormat::RGB16I,
            GlBaseFormat::RGB_INTEGER,
            GlDataType::SHORT,
        ),
        (
            GlFormat::RGB16UI,
            GlBaseFormat::RGB_INTEGER,
            GlDataType::UNSIGNED_SHORT,
        ),
        (GlFormat::RGB32F, GlBaseFormat::RGB, GlDataType::FLOAT),
        (GlFormat::RGB32I, GlBaseFormat::RGB_INTEGER, GlDataType::INT),
        (
            GlFormat::RGB32UI,
            GlBaseFormat::RGB_INTEGER,
            GlDataType::UNSIGNED_INT,
        ),
        (
            GlFormat::RGB5_A1,
            GlBaseFormat::BGRA,
            GlDataType::UNSIGNED_SHORT_1_5_5_5_REV,
        ),
        (
            GlFormat::RGB5_A1,
            GlBaseFormat::BGRA,
            GlDataType::UNSIGNED_SHORT_5_5_5_1,
        ),
        (
            GlFormat::RGB5_A1,
            GlBaseFormat::RGBA,
            GlDataType::UNSIGNED_SHORT_5_5_5_1,
        ),
        (
            GlFormat::RGB565,
            GlBaseFormat::RGB,
            GlDataType::UNSIGNED_SHORT_5_6_5_REV,
        ),
        (
            GlFormat::RGB565,
            GlBaseFormat::RGB,
            GlDataType::UNSIGNED_SHORT_5_6_5,
        ),
        (GlFormat::RGB8, GlBaseFormat::BGR, GlDataType::UNSIGNED_BYTE),
        (GlFormat::RGB8, GlBaseFormat::RGB, GlDataType::UNSIGNED_BYTE),
        (GlFormat::RGB8_SNORM, GlBaseFormat::BGR, GlDataType::BYTE),
        (GlFormat::RGB8_SNORM, GlBaseFormat::RGB, GlDataType::BYTE),
        (GlFormat::RGB8I, GlBaseFormat::BGR_INTEGER, GlDataType::BYTE),
        (GlFormat::RGB8I, GlBaseFormat::RGB_INTEGER, GlDataType::BYTE),
        (
            GlFormat::RGB8UI,
            GlBaseFormat::BGR_INTEGER,
            GlDataType::UNSIGNED_BYTE,
        ),
        (
            GlFormat::RGB8UI,
            GlBaseFormat::RGB_INTEGER,
            GlDataType::UNSIGNED_BYTE,
        ),
        (
            GlFormat::RGB9_E5,
            GlBaseFormat::RGB,
            GlDataType::UNSIGNED_INT_5_9_9_9_REV,
        ),
        (
            GlFormat::RGBA16,
            GlBaseFormat::RGBA,
            GlDataType::UNSIGNED_SHORT,
        ),
        (
            GlFormat::RGBA16_SNORM,
            GlBaseFormat::RGBA,
            GlDataType::SHORT,
        ),
        (
            GlFormat::RGBA16F,
            GlBaseFormat::RGBA,
            GlDataType::HALF_FLOAT,
        ),
        (
            GlFormat::RGBA16I,
            GlBaseFormat::RGBA_INTEGER,
            GlDataType::SHORT,
        ),
        (
            GlFormat::RGBA16UI,
            GlBaseFormat::RGBA_INTEGER,
            GlDataType::UNSIGNED_SHORT,
        ),
        (GlFormat::RGBA32F, GlBaseFormat::RGBA, GlDataType::FLOAT),
        (
            GlFormat::RGBA32I,
            GlBaseFormat::RGBA_INTEGER,
            GlDataType::INT,
        ),
        (
            GlFormat::RGBA32UI,
            GlBaseFormat::RGBA_INTEGER,
            GlDataType::UNSIGNED_INT,
        ),
        (
            GlFormat::RGBA4,
            GlBaseFormat::BGRA,
            GlDataType::UNSIGNED_SHORT_4_4_4_4,
        ),
        (
            GlFormat::RGBA4,
            GlBaseFormat::RGBA,
            GlDataType::UNSIGNED_SHORT_4_4_4_4,
        ),
        (
            GlFormat::RGBA8,
            GlBaseFormat::BGRA,
            GlDataType::UNSIGNED_BYTE,
        ),
        (
            GlFormat::RGBA8,
            GlBaseFormat::RGBA,
            GlDataType::UNSIGNED_BYTE,
        ),
        (GlFormat::RGBA8_SNORM, GlBaseFormat::BGRA, GlDataType::BYTE),
        (GlFormat::RGBA8_SNORM, GlBaseFormat::RGBA, GlDataType::BYTE),
        (
            GlFormat::RGBA8I,
            GlBaseFormat::BGRA_INTEGER,
            GlDataType::BYTE,
        ),
        (
            GlFormat::RGBA8I,
            GlBaseFormat::RGBA_INTEGER,
            GlDataType::BYTE,
        ),
        (
            GlFormat::RGBA8UI,
            GlBaseFormat::BGRA_INTEGER,
            GlDataType::UNSIGNED_BYTE,
        ),
        (
            GlFormat::RGBA8UI,
            GlBaseFormat::RGBA_INTEGER,
            GlDataType::UNSIGNED_BYTE,
        ),
        (
            GlFormat::SRGB8,
            GlBaseFormat::BGR,
            GlDataType::UNSIGNED_BYTE,
        ),
        (
            GlFormat::SRGB8,
            GlBaseFormat::RGB,
            GlDataType::UNSIGNED_BYTE,
        ),
        (
            GlFormat::SRGB8_ALPHA8,
            GlBaseFormat::BGRA,
            GlDataType::UNSIGNED_BYTE,
        ),
        (
            GlFormat::SRGB8_ALPHA8,
            GlBaseFormat::RGBA,
            GlDataType::UNSIGNED_BYTE,
        ),
        (
            GlFormat::STENCIL_INDEX8,
            GlBaseFormat::STENCIL_INDEX,
            GlDataType::UNSIGNED_BYTE,
        ),
    ];
}
