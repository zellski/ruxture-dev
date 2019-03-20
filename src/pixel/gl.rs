use super::{CompContent, CompLayout};

use GlBaseFormat::*;
use GlDataType::*;

// TODO: should support, or at minimum detect and reject, formats from OpenGL < 4.0
// TODO: should support a few OpenGL ES 3.0 formats, like GL_SRG8_EXT

pub fn gl_upgrade_old_formats(format: GlFormat) -> GlFormat {
  match format {
    GlFormat::LUMINANCE => GlFormat::R8,
    GlFormat::LUMINANCE_ALPHA => GlFormat::RG8,
    GlFormat::RGB => GlFormat::RGB8,
    GlFormat::RGBA => GlFormat::RGBA8,
    _ => format,
  }
}

pub fn to_gl(layout: &CompLayout, content: &CompContent) -> Option<(GlBaseFormat, GlDataType)> {
  let by_content = |comp: GlBaseFormat,
                    comp_int: GlBaseFormat,
                    u_type: GlDataType,
                    s_type: GlDataType,
                    f_type: Option<GlDataType>| {
    match *content {
      CompContent::UNORM => Some((comp, u_type)),
      CompContent::SRGB => Some((comp, u_type)),
      CompContent::SNORM => Some((comp, s_type)),
      CompContent::SFLOAT => f_type.and_then(|tt| Some((comp, tt))),
      CompContent::UINT => Some((comp_int, u_type)),
      CompContent::SINT => Some((comp_int, s_type)),
      _ => panic!("Internal error: unexpected comp_content: {:?}", *content),
    }
  };
  match *layout {
    CompLayout::A8 => Some((ALPHA, UNSIGNED_BYTE)),
    CompLayout::R8 => by_content(RED, RED_INTEGER, UNSIGNED_BYTE, BYTE, None),
    CompLayout::R16 => by_content(RED, RED_INTEGER, UNSIGNED_SHORT, SHORT, Some(HALF_FLOAT)),
    CompLayout::R32 => by_content(RED, RED_INTEGER, UNSIGNED_INT, INT, Some(FLOAT)),
    CompLayout::R8G8 => by_content(RG, RG_INTEGER, UNSIGNED_BYTE, BYTE, None),
    CompLayout::R16G16 => by_content(RG, RG_INTEGER, UNSIGNED_SHORT, SHORT, Some(HALF_FLOAT)),
    CompLayout::R32G32 => by_content(RG, RG_INTEGER, UNSIGNED_INT, INT, Some(FLOAT)),
    CompLayout::R8G8B8 => by_content(RGB, RGB_INTEGER, UNSIGNED_BYTE, BYTE, None),
    CompLayout::B8G8R8 => by_content(BGR, BGR_INTEGER, UNSIGNED_BYTE, BYTE, None),
    CompLayout::R16G16B16 => by_content(RGB, RGB_INTEGER, UNSIGNED_SHORT, SHORT, Some(HALF_FLOAT)),
    CompLayout::R32G32B32 => by_content(RGB, RGB_INTEGER, UNSIGNED_INT, INT, Some(FLOAT)),
    CompLayout::R8G8B8A8 => by_content(RGBA, RGBA_INTEGER, UNSIGNED_BYTE, BYTE, None),
    CompLayout::B8G8R8A8 => by_content(BGRA, BGRA_INTEGER, UNSIGNED_BYTE, BYTE, None),
    CompLayout::R16G16B16A16 => {
      by_content(RGBA, RGBA_INTEGER, UNSIGNED_SHORT, SHORT, Some(HALF_FLOAT))
    }
    CompLayout::R32G32B32A32 => by_content(RGBA, RGBA_INTEGER, UNSIGNED_INT, INT, Some(FLOAT)),

    CompLayout::R5G6B5 => Some((RGB, UNSIGNED_SHORT_5_6_5)),
    CompLayout::B5G6R5 => Some((RGB, UNSIGNED_SHORT_5_6_5_REV)),
    CompLayout::R4G4B4A4 => Some((RGBA, UNSIGNED_SHORT_4_4_4_4)),
    CompLayout::B4G4R4A4 => Some((BGRA, UNSIGNED_SHORT_4_4_4_4)),
    CompLayout::R5G5B5A1 => Some((RGBA, UNSIGNED_SHORT_5_5_5_1)),
    CompLayout::B5G5R5A1 => Some((BGRA, UNSIGNED_SHORT_5_5_5_1)),
    CompLayout::A1R5G5B5 => Some((BGRA, UNSIGNED_SHORT_1_5_5_5_REV)),
    CompLayout::A2R10G10B10 => Some((
      if *content == CompContent::UINT {
        BGRA_INTEGER
      } else {
        BGRA
      },
      UNSIGNED_INT_2_10_10_10_REV,
    )),
    CompLayout::A2B10G10R10 => Some((
      if *content == CompContent::UINT {
        RGBA_INTEGER
      } else {
        RGBA
      },
      UNSIGNED_INT_2_10_10_10_REV,
    )),
    CompLayout::B10G11R11 => Some((RGB, UNSIGNED_INT_10F_11F_11F_REV)),
    CompLayout::E5B9G9R9 => Some((RGB, UNSIGNED_INT_5_9_9_9_REV)),
    CompLayout::S8 => Some((STENCIL_INDEX, UNSIGNED_BYTE)),
    CompLayout::D16 => Some((DEPTH_COMPONENT, UNSIGNED_SHORT)),
    CompLayout::D32 => Some((DEPTH_COMPONENT, FLOAT)),
    CompLayout::D24S8 => Some((DEPTH_STENCIL, UNSIGNED_INT_24_8)),
    CompLayout::D32S8 => Some((DEPTH_STENCIL, FLOAT_32_UNSIGNED_INT_24_8_REV)),
    CompLayout::X8D24 => Some((DEPTH_COMPONENT, UNSIGNED_INT)),
    // layouts without OpenGL equivalents
    CompLayout::R64
    | CompLayout::R64G64
    | CompLayout::A8B8G8R8
    | CompLayout::R64G64B64
    | CompLayout::R11
    | CompLayout::R11G11
    | CompLayout::R64G64B64A64
    | CompLayout::D16S8 => None,
  }
}

#[allow(non_camel_case_types)]
#[derive(Primitive, Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum GlFormat {
  ETC1_RGB8_OES = 0x8D64,
  COMPRESSED_R11_EAC = 0x9270,
  COMPRESSED_SIGNED_R11_EAC = 0x9271,
  COMPRESSED_RG11_EAC = 0x9272,
  COMPRESSED_SIGNED_RG11_EAC = 0x9273,
  COMPRESSED_RGB8_ETC2 = 0x9274,
  COMPRESSED_SRGB8_ETC2 = 0x9275,
  COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2 = 0x9276,
  COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2 = 0x9277,
  COMPRESSED_RGBA8_ETC2_EAC = 0x9278,
  COMPRESSED_SRGB8_ALPHA8_ETC2_EAC = 0x9279,
  COMPRESSED_RGB_PVRTC_4BPPV1_IMG = 0x8C00,
  COMPRESSED_RGB_PVRTC_2BPPV1_IMG = 0x8C01,
  COMPRESSED_RGBA_PVRTC_4BPPV1_IMG = 0x8C02,
  COMPRESSED_RGBA_PVRTC_2BPPV1_IMG = 0x8C03,
  COMPRESSED_RGBA_PVRTC_2BPPV2_IMG = 0x9137,
  COMPRESSED_RGBA_PVRTC_4BPPV2_IMG = 0x9138,
  COMPRESSED_RGB_S3TC_DXT1_EXT = 0x83F0,
  COMPRESSED_RGBA_S3TC_DXT1_EXT = 0x83F1,
  COMPRESSED_RGBA_S3TC_DXT3_EXT = 0x83F2,
  COMPRESSED_RGBA_S3TC_DXT5_EXT = 0x83F3,
  COMPRESSED_SRGB_S3TC_DXT1_EXT = 0x8C4C,
  COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT = 0x8C4D,
  COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT = 0x8C4E,
  COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT = 0x8C4F,
  COMPRESSED_LUMINANCE_LATC1_EXT = 0x8C70,
  COMPRESSED_LUMINANCE_ALPHA_LATC2_EXT = 0x8C72,
  COMPRESSED_RGBA_BPTC_UNORM_ARB = 0x8E8C,
  COMPRESSED_SRGB_ALPHA_BPTC_UNORM_ARB = 0x8E8D,
  COMPRESSED_RGB_BPTC_SIGNED_FLOAT_ARB = 0x8E8E,
  COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_ARB = 0x8E8F,
  COMPRESSED_SRGB_PVRTC_2BPPV1_EXT = 0x8A54,
  COMPRESSED_SRGB_PVRTC_4BPPV1_EXT = 0x8A55,
  COMPRESSED_SRGB_ALPHA_PVRTC_2BPPV1_EXT = 0x8A56,
  COMPRESSED_SRGB_ALPHA_PVRTC_4BPPV1_EXT = 0x8A57,
  ATC_RGB_AMD = 0x8C92,
  ATC_RGBA_EXPLICIT_ALPHA_AMD = 0x8C93,
  ATC_RGBA_INTERPOLATED_ALPHA_AMD = 0x87EE,
  COMPRESSED_RGBA_ASTC_4x4_KHR = 0x93B0,
  COMPRESSED_RGBA_ASTC_5x5_KHR = 0x93B2,
  COMPRESSED_RGBA_ASTC_6x6_KHR = 0x93B4,
  COMPRESSED_RGBA_ASTC_8x5_KHR = 0x93B5,
  COMPRESSED_RGBA_ASTC_8x6_KHR = 0x93B6,
  COMPRESSED_RGBA_ASTC_10x5_KHR = 0x93B8,
  COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR = 0x93D0,
  COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR = 0x93D2,
  COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR = 0x93D4,
  COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR = 0x93D5,
  COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR = 0x93D6,
  COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR = 0x93D8,

  // OpenGL ES 2.0 had format = internal_format
  ALPHA = 0x1906,
  RGB = 0x1907,
  RGBA = 0x1908,
  LUMINANCE = 0x1909,
  LUMINANCE_ALPHA = 0x190A,

  SRGB8_ALPHA8 = 0x8C43,
  SRGB8 = 0x8C41,
  SR8_EXT = 0x8FBD,
  RGBA8UI = 0x8D7C,
  RGBA8I = 0x8D8E,
  RGBA8_SNORM = 0x8F97,
  RGBA8 = 0x8058,
  RGBA4 = 0x8056,
  RGBA32UI = 0x8D70,
  RGBA32I = 0x8D82,
  RGBA32F = 0x8814,
  RGBA16UI = 0x8D76,
  RGBA16I = 0x8D88,
  RGBA16F = 0x881A,
  RGBA16_SNORM = 0x8F9B,
  RGBA16 = 0x805B,
  RGB9_E5 = 0x8C3D,
  RGB8UI = 0x8D7D,
  RGB8I = 0x8D8F,
  RGB8_SNORM = 0x8F96,
  RGB8 = 0x8051,
  RGB565 = 0x8D62,
  RGB5_A1 = 0x8057,
  RGB32UI = 0x8D71,
  RGB32I = 0x8D83,
  RGB32F = 0x8815,
  RGB16UI = 0x8D77,
  RGB16I = 0x8D89,
  RGB16F = 0x881B,
  RGB16_SNORM = 0x8F9A,
  RGB16 = 0x8054,
  RGB10_A2 = 0x8059,
  RGB10_A2UI = 0x906F,
  RG8UI = 0x8238,
  RG8I = 0x8237,
  RG8_SNORM = 0x8F95,
  RG8 = 0x822B,
  RG32UI = 0x823C,
  RG32I = 0x823B,
  RG32F = 0x8230,
  RG16UI = 0x823A,
  RG16I = 0x8239,
  RG16F = 0x822F,
  RG16_SNORM = 0x8F99,
  RG16 = 0x822C,
  R8UI = 0x8232,
  R8I = 0x8231,
  R8_SNORM = 0x8F94,
  R8 = 0x8229,
  R32UI = 0x8236,
  R32I = 0x8235,
  R32F = 0x822E,
  R16UI = 0x8234,
  R16I = 0x8233,
  R16F = 0x822D,
  R16_SNORM = 0x8F98,
  R16 = 0x822A,
  R11F_G11F_B10F = 0x8C3A,
  A8 = 0x803C,
  STENCIL_INDEX8 = 0x8D48,
  DEPTH_COMPONENT16 = 0x81A5,
  DEPTH_COMPONENT24 = 0x81A6,
  DEPTH_COMPONENT32 = 0x81A7,
  DEPTH24_STENCIL8 = 0x88F0,
  DEPTH_COMPONENT32F = 0x8CAC,
  DEPTH32F_STENCIL8 = 0x8CAD,
}

#[allow(dead_code, non_camel_case_types)]
#[derive(Primitive, Copy, Clone, PartialEq, Eq, Debug)]
pub enum GlBaseFormat {
  STENCIL_INDEX = 0x1901,
  DEPTH_COMPONENT = 0x1902,
  DEPTH_STENCIL = 0x84F9,

  RED = 0x1903,
  GREEN = 0x1904,
  BLUE = 0x1905,
  ALPHA = 0x1906,

  RG = 0x8227,
  RGB = 0x1907,
  RGBA = 0x1908,
  BGR = 0x80E0,
  BGRA = 0x80E1,

  RED_INTEGER = 0x8D94,
  GREEN_INTEGER = 0x8D95,
  BLUE_INTEGER = 0x8D96,

  RGBA_INTEGER = 0x8D99,
  RGB_INTEGER = 0x8D98,
  RG_INTEGER = 0x8228,
  BGR_INTEGER = 0x8D9A,
  BGRA_INTEGER = 0x8D9B,
}

#[allow(dead_code, non_camel_case_types)]
#[derive(Primitive, Copy, Clone, PartialEq, Eq, Debug)]
pub enum GlDataType {
  BYTE = 0x1400,
  UNSIGNED_BYTE = 0x1401,
  SHORT = 0x1402,
  UNSIGNED_SHORT = 0x1403,
  INT = 0x1404,
  UNSIGNED_INT = 0x1405,
  FLOAT = 0x1406,
  HALF_FLOAT = 0x140B,

  UNSIGNED_BYTE_3_3_2 = 0x8032,
  UNSIGNED_BYTE_2_3_3_REV = 0x8362,
  UNSIGNED_SHORT_5_6_5 = 0x8363,
  UNSIGNED_SHORT_5_6_5_REV = 0x8364,
  UNSIGNED_SHORT_4_4_4_4 = 0x8033,
  UNSIGNED_SHORT_4_4_4_4_REV = 0x8365,
  UNSIGNED_SHORT_5_5_5_1 = 0x8034,
  UNSIGNED_SHORT_1_5_5_5_REV = 0x8366,
  UNSIGNED_INT_8_8_8_8 = 0x8035,
  UNSIGNED_INT_8_8_8_8_REV = 0x8367,
  UNSIGNED_INT_10_10_10_2 = 0x8036,
  UNSIGNED_INT_2_10_10_10_REV = 0x8368,
  UNSIGNED_INT_24_8 = 0x84FA,
  UNSIGNED_INT_10F_11F_11F_REV = 0x8C3B,
  UNSIGNED_INT_5_9_9_9_REV = 0x8C3E,
  FLOAT_32_UNSIGNED_INT_24_8_REV = 0x8DAD,
}
