pub mod gl;
use gl::GlFormat;

pub mod dxt10;
use dxt10::Dxt10Format;

pub mod vulkan;
use vulkan::VkFormat;

mod db;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ColourSpace {
    Linear,
    sRGB,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BaseFormat {
    Red,
    RG,
    RGB,
    RGBA,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Dimensions(u32, u32, u32);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PixelFormat {
    pub name: &'static str,
    pub base_format: BaseFormat,
    pub colspace: ColourSpace,
    pub block_dim: Option<Dimensions>,
    pub vk_format: Option<VkFormat>,
    pub gl_format: Option<GlFormat>,
    pub four_cc: Option<u32>,
    pub dxt10_format: Option<Dxt10Format>,
}

impl PixelFormat {
    pub fn for_gl_format(gl_format: GlFormat) -> Option<PixelFormat> {
        db::PIXEL_FORMATS
            .iter()
            .find(|x| x.gl_format.map_or(false, |f| f == gl_format))
            .map(|g| *g)
    }

    pub fn with_gl(&self, gl_format: GlFormat) -> PixelFormat {
        PixelFormat {
            gl_format: Some(gl_format),
            ..*self
        }
    }
    pub fn with_four_cc(&self, bytes: &[u8; 4]) -> PixelFormat {
        let four_cc = Some(
            (bytes[0] as u32)
                | (bytes[1] as u32) << 8
                | (bytes[2] as u32) << 16
                | (bytes[3] as u32) << 24,
        );
        PixelFormat { four_cc, ..*self }
    }
    pub fn with_dxt10(&self, dxt10_format: Dxt10Format) -> PixelFormat {
        PixelFormat {
            dxt10_format: Some(dxt10_format),
            ..*self
        }
    }
    pub fn with_vulkan(&self, vk_format: VkFormat) -> PixelFormat {
        PixelFormat {
            vk_format: Some(vk_format),
            ..*self
        }
    }
}

// const MAPPING: [KtxMapping; 73] = [
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGBA_S3TC_DXT1_EXT,
//         srgb_internal_format: COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGBA_S3TC_DXT3_EXT,
//         srgb_internal_format: COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGBA_S3TC_DXT5_EXT,
//         srgb_internal_format: COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_LUMINANCE_LATC1_EXT,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RED,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_LUMINANCE_ALPHA_LATC2_EXT,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RG,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGB_BPTC_SIGNED_FLOAT_ARB,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGB,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGBA_BPTC_UNORM_ARB,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: ETC1_RGB8_OES,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGB,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGB8_ETC2,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGB,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGBA8_ETC2_EAC,
//         srgb_internal_format: COMPRESSED_SRGB8_ETC2,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2,
//         srgb_internal_format: COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGB_PVRTC_2BPPV1_IMG,
//         srgb_internal_format: COMPRESSED_SRGB_PVRTC_2BPPV1_EXT,
//         base_internal_format: RGB,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGB_PVRTC_4BPPV1_IMG,
//         srgb_internal_format: COMPRESSED_SRGB_PVRTC_4BPPV1_EXT,
//         base_internal_format: RGB,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGBA_PVRTC_2BPPV1_IMG,
//         srgb_internal_format: COMPRESSED_SRGB_ALPHA_PVRTC_2BPPV1_EXT,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGBA_PVRTC_4BPPV1_IMG,
//         srgb_internal_format: COMPRESSED_SRGB_ALPHA_PVRTC_4BPPV1_EXT,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGBA_PVRTC_2BPPV2_IMG,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGBA_PVRTC_4BPPV2_IMG,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: ATC_RGB_AMD,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGB,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: ATC_RGBA_EXPLICIT_ALPHA_AMD,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: ATC_RGBA_INTERPOLATED_ALPHA_AMD,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGBA_ASTC_4x4_KHR,
//         srgb_internal_format: COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGBA_ASTC_5x5_KHR,
//         srgb_internal_format: COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGBA_ASTC_6x6_KHR,
//         srgb_internal_format: COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGBA_ASTC_8x5_KHR,
//         srgb_internal_format: COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGBA_ASTC_8x6_KHR,
//         srgb_internal_format: COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: COMPRESSED_RGBA_ASTC_10x5_KHR,
//         srgb_internal_format: COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR,
//         base_internal_format: RGBA,
//         data_type: KtxDataType::NONE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: KtxFormat::ALPHA,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: KtxBaseFormat::ALPHA,
//         data_type: UNSIGNED_BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: R8,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RED,
//         data_type: UNSIGNED_BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: R8I,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RED,
//         data_type: BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: R8UI,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RED,
//         data_type: UNSIGNED_BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: R8_SNORM,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RED,
//         data_type: BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: R16,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RED,
//         data_type: UNSIGNED_SHORT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: R16I,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RED,
//         data_type: SHORT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: R16UI,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RED,
//         data_type: UNSIGNED_SHORT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: R16F,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RED,
//         data_type: HALF_FLOAT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: R16_SNORM,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RED,
//         data_type: SHORT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: R32I,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RED,
//         data_type: INT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: R32UI,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RED,
//         data_type: UNSIGNED_INT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: R32F,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RED,
//         data_type: FLOAT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RG8,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RG,
//         data_type: UNSIGNED_BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RG8I,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RG,
//         data_type: BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RG8UI,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RG,
//         data_type: UNSIGNED_BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RG8_SNORM,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RG,
//         data_type: BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RG16,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RG,
//         data_type: UNSIGNED_SHORT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RG16I,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RG,
//         data_type: SHORT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RG16UI,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RG,
//         data_type: UNSIGNED_SHORT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RG16F,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RG,
//         data_type: FLOAT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RG16_SNORM,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RG,
//         data_type: SHORT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RG32I,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RG,
//         data_type: INT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RG32UI,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RG,
//         data_type: UNSIGNED_INT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RG32F,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RG,
//         data_type: FLOAT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGB8,
//         srgb_internal_format: SRGB8,
//         base_internal_format: RGB,
//         data_type: UNSIGNED_BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGB8I,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGB,
//         data_type: BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGB8UI,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGB,
//         data_type: UNSIGNED_BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGB8_SNORM,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGB,
//         data_type: BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGB9_E5,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGB,
//         data_type: UNSIGNED_INT_5_9_9_9_REV,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGBA8,
//         srgb_internal_format: SRGB8_ALPHA8,
//         base_internal_format: RGBA,
//         data_type: UNSIGNED_BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGBA8I,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGBA8UI,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: UNSIGNED_BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGBA8_SNORM,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: BYTE,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGBA16,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: UNSIGNED_SHORT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGBA16I,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: SHORT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGBA16UI,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: UNSIGNED_SHORT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGBA16F,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: HALF_FLOAT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGBA16_SNORM,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: SHORT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGBA32I,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: INT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGBA32UI,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: UNSIGNED_INT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGBA32F,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: FLOAT,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGB565,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGB,
//         data_type: UNSIGNED_SHORT_5_6_5,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGBA4,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: UNSIGNED_SHORT_4_4_4_4,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGB5_A1,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: UNSIGNED_SHORT_5_5_5_1,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: RGB10_A2,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGBA,
//         data_type: UNSIGNED_INT_2_10_10_10_REV,
//     },
//     KtxMapping {
//         format: BC1,
//         internal_format: R11F_G11F_B10F,
//         srgb_internal_format: KtxFormat::NONE,
//         base_internal_format: RGB,
//         data_type: UNSIGNED_INT_10F_11F_11F_REV,
//     },
