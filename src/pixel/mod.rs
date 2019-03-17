pub mod gl;
use gl::{GlBaseFormat, GlDataType, GlFormat};

pub mod dxt10;
use dxt10::Dxt10Format;

pub mod vulkan;
use vulkan::VkFormat;

mod db;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum CompLayout {
    R8,
    R11,
    R16,
    R32,
    R64,
    R8G8,
    R11G11,
    R16G16,
    R32G32,
    R64G64,
    R5G6B5,
    R8G8B8,
    R16G16B16,
    R32G32B32,
    R64G64B64,
    R4G4B4A4,
    R5G5B5A1,
    R8G8B8A8,
    R16G16B16A16,
    R32G32B32A32,
    R64G64B64A64,
    B8G8R8,
    B10G11R11,
    B4G4R4A4,
    B5G6R5,
    B5G5R5A1,
    B8G8R8A8,
    A1R5G5B5,
    A2R10G10B10,
    A2B10G10R10,
    A8B8G8R8,
    E5B9G9R9,
    S8,
    D16,
    D32,
    D16S8,
    D24S8,
    D32S8,
    X8D24,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum CompContent {
    UNORM,
    SNORM,
    SFLOAT,
    UFLOAT,
    UINT,
    SINT,
    SRGB,
    SPECIAL,
}

// pub fn to_gl(layout: &CompLayout, content: &CompContent) -> GlBaseFormat {
//     let int_or_not = |a, b| match *content {
//         CompContent::UNORM | CompContent::SNORM | CompContent::SFLOAT | CompContent::SRGB => a,
//         CompContent::UINT | CompContent::SINT => b,
//     };

//     match *layout {
//         CompLayout::R8 | CompLayout::R16 | CompLayout::R32 | CompLayout::R64 => {
//             int_or_not(GlBaseFormat::RED, GlBaseFormat::RED_INTEGER)
//         }
//         CompLayout::R8G8 | CompLayout::R16G16 | CompLayout::R32G32 | CompLayout::R64G64 => {
//             int_or_not(GlBaseFormat::RG, GlBaseFormat::RG_INTEGER)
//         }
//         CompLayout::R5G6B5
//         | CompLayout::R8G8B8
//         | CompLayout::R16G16B16
//         | CompLayout::R32G32B32
//         | CompLayout::R64G64B64 => int_or_not(GlBaseFormat::RGB, GlBaseFormat::RGB_INTEGER),

//         CompLayout::R4G4B4A4
//         | CompLayout::R5G5B5A1
//         | CompLayout::R8G8B8A8
//         | CompLayout::R16G16B16A16
//         | CompLayout::R32G32B32A32
//         | CompLayout::R64G64B64A64 => int_or_not(GlBaseFormat::RGBA, GlBaseFormat::RGBA_INTEGER),

//         CompLayout::A1R5G5B5 | CompLayout::A2R10G10B10 => GlBaseFormat::BGRA,
//     }
// }

// CompLayout::B8G8R8 => (GlBaseFormat::X, GlDataType::Y),
// CompLayout::B10G11R11 => (GlBaseFormat::X, GlDataType::Y),
// CompLayout::B4G4R4A4 => (GlBaseFormat::X, GlDataType::Y),
// CompLayout::B5G6R5 => (GlBaseFormat::X, GlDataType::Y),
// CompLayout::B5G5R5A1 => (GlBaseFormat::X, GlDataType::Y),
// CompLayout::B8G8R8A8 => (GlBaseFormat::X, GlDataType::Y),
// CompLayout::A8B8G8R8 => (GlBaseFormat::X, GlDataType::Y),
// CompLayout::E5B9G9R9 => (GlBaseFormat::X, GlDataType::Y),
// CompLayout::S8 => (GlBaseFormat::X, GlDataType::Y),
// CompLayout::D16 => (GlBaseFormat::X, GlDataType::Y),
// CompLayout::D32 => (GlBaseFormat::X, GlDataType::Y),
// CompLayout::D16S8 => (GlBaseFormat::X, GlDataType::Y),
// CompLayout::D24S8 => (GlBaseFormat::X, GlDataType::Y),
// CompLayout::D32S8 => (GlBaseFormat::X, GlDataType::Y),
// CompLayout::X8D24 => (GlBaseFormat::X, GlDataType::Y),

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Dimensions(u32, u32, u32);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PixelFormat {
    pub tag: &'static str,
    pub comp_layout: CompLayout,
    pub comp_content: CompContent,
    pub block_dim: Option<Dimensions>,
    pub vk_format: Option<VkFormat>,
    pub gl_format: Option<GlFormat>,
    pub four_cc: Option<u32>,
    pub dxt10_format: Option<Dxt10Format>,
}

impl std::fmt::Display for PixelFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "PixFormat({} - {:?}, {:?})",
            self.tag, self.comp_layout, self.comp_content
        )
    }
}

impl PixelFormat {
    pub fn for_gl_format(gl_format: GlFormat) -> Vec<&'static PixelFormat> {
        db::PIXEL_FORMATS
            .iter()
            .filter(|x| x.gl_format.map_or(false, |f| f == gl_format))
            .collect()
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
//         base_internal_format: KtxCompLayout::ALPHA,
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
