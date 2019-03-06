extern crate clap;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;

use ruxture::file::FileFormat;
use ruxture::*;

use clap::{App, Arg};

fn main() {
    let TODO_clean_up_options_and_results = true;

    let matches = App::new("Ruxture")
        .version("0.1.0")
        .author("Pär Winzell <par.winzell@alyx.com>")
        .about("Texture file format conversion and exploration.")
        .arg(
            Arg::with_name("in_file")
                .required(true)
                .short("i")
                .long("input")
                .takes_value(true)
                .help("Input file to read"),
        )
        .arg(
            Arg::with_name("out_file")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("Output file to write"),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .takes_value(true)
                .possible_values(&FileFormat::variants())
                .help("Texture file format"),
        )
        .get_matches();

    let in_file = matches.value_of("in_file").unwrap();

    let format = matches
        .value_of("format")
        .and_then(|format_str| Some(FileFormat::from_str(format_str).unwrap()));

    match matches.value_of("out_file") {
        None => identify_file(in_file),
        Some(out_file) => convert_file(in_file, format, out_file),
    }
}

fn identify_file(in_file: &str) {
    let in_path = Path::new(in_file);
    let contents = read_and_parse(in_path);
    println!("Pixel Format: {:?}", contents.format);
    println!("Texture Dimensions: {:?}", contents.dim);
}

fn convert_file(in_file: &str, format: Option<FileFormat>, out_file: &str) {
    let texture = read_and_parse(Path::new(in_file));

    let out_path = Path::new(out_file);
    let format = format.unwrap_or_else(|| format_from_path(out_path));

    let bytes_written = generate_and_write(&texture, format, out_path);
    println!("Wrote {} bytes to {}.", bytes_written, out_file);
}

fn format_from_path(out_path: &Path) -> FileFormat {
    if let Some(ext) = out_path.extension() {
        match ext.to_str() {
            Some("dds") => FileFormat::DDS,
            _ => FileFormat::DDS,
        }
    } else {
        println!(
            "Can't figure texture format from output path: {}",
            out_path.display(),
        );
        std::process::exit(1);
    }
}

fn generate_and_write(texture: &Texture, format: FileFormat, out_path: &Path) -> usize {
    let (generated_format, file_blob) = match ruxture::generate(&texture, format) {
        Err(why) => {
            println!("Failed to generate texture:\n{}", why.description());
            std::process::exit(1);
        }
        Ok(blob) => blob,
    };
    assert_eq!(format, generated_format);

    let mut file = match File::create(&out_path) {
        Err(why) => {
            println!(
                "Failed to create file {} for writing:\n{}",
                out_path.display(),
                why.description(),
            );
            std::process::exit(1);
        }
        Ok(file) => file,
    };

    match file.write(&file_blob) {
        Err(why) => {
            println!(
                "Failed to write data to file {}:\n{}",
                out_path.display(),
                why.description(),
            );
            std::process::exit(1);
        }
        Ok(bytes) => bytes,
    }
}

fn read_and_parse(in_path: &Path) -> Texture {
    let mut file = match File::open(&in_path) {
        Err(why) => {
            println!(
                "Failed to open file {} for reading:\n{}",
                in_path.display(),
                why.description(),
            );
            std::process::exit(1);
        }
        Ok(file) => file,
    };

    let mut buf = Vec::new();
    if let Err(why) = file.read_to_end(&mut buf) {
        println!(
            "Failed to read data from file {}:\n{}",
            in_path.display(),
            why.description(),
        );
        std::process::exit(1);
    }

    match ruxture::parse(&buf) {
        Err(why) => {
            println!(
                "Failed to parse contents of file {}:\n{}",
                in_path.display(),
                why.description(),
            );
            std::process::exit(1);
        }
        Ok(texture) => texture,
    }
}
