use crate::*;

mod ktx1;

clap::arg_enum! {
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FileFormat {
    KTX1,
    DDS,
}
}

pub const FILE_FORMATS: [FileFormat; 2] = [FileFormat::KTX1, FileFormat::DDS];

pub trait FileCodec {
    fn claim_for_parsing(&self, contents: &FileBlob) -> bool;
    fn parse(&self, contents: &FileBlob) -> RuxResult<Texture>;
    fn generate(&self, texture: &Texture, format: FileFormat) -> RuxResult<(FileFormat, FileBlob)>;
}

impl FileFormat {
    pub fn extension(&self) -> &str {
        match *self {
            FileFormat::KTX1 => "ktx",
            FileFormat::DDS => "dds",
        }
    }

    pub fn codec(&self) -> Option<Box<FileCodec>> {
        match *self {
            FileFormat::KTX1 => Some(Box::from(ktx1::Ktx1Codec {})),
            _ => None,
        }
    }
}
