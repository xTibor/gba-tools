extern crate docopt;
extern crate gba_rs;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use gba_rs::compression::Compressor;
use gba_rs::compression::bios::{RleCompressor, Lz77Compressor, Diff8Filter, Diff16Filter};
use gba_rs::compression::game_specific::wario_land_4::{Wl4RleCompressor, Wl4Rle8Compressor, Wl4Rle16Compressor};
use gba_rs::utils::streams::{InputStream, OutputStream};
use std::io::{Read, Write};

const USAGE: &'static str = "
Usage:
    gba-compress --type <type> [--input <input>] [--output <output>] [--decompress]
    gba-compress --help

Options:
    -i, --input <input>    Input file
    -o, --output <output>  Output file
    -t, --type <type>      Compression type
    -d, --decompress       Decompression
    -h, --help             Display this message
";

#[derive(Debug, Deserialize, Copy, Clone)]
enum CompressionType {
    Diff8,
    Diff16,
    Lz77,
    Rle,
    Wl4Rle,
    Wl4Rle8,
    Wl4Rle16,
}

#[derive(Debug, Deserialize)]
struct Args {
    flag_type: CompressionType,
    flag_input: Option<String>,
    flag_output: Option<String>,
    flag_decompress: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut input = InputStream::new(args.flag_input).unwrap();
    let mut output = OutputStream::new(args.flag_output).unwrap();

    let compressor: Box<Compressor> = match args.flag_type {
        CompressionType::Diff8 => Box::new(Diff8Filter::default()),
        CompressionType::Diff16 => Box::new(Diff16Filter::default()),
        CompressionType::Lz77 => Box::new(Lz77Compressor::default()),
        CompressionType::Rle => Box::new(RleCompressor::default()),
        CompressionType::Wl4Rle => Box::new(Wl4RleCompressor::default()),
        CompressionType::Wl4Rle8 => Box::new(Wl4Rle8Compressor::default()),
        CompressionType::Wl4Rle16 => Box::new(Wl4Rle16Compressor::default()),
    };

    let mut input_data = Vec::new();
    let mut output_data = Vec::new();

    input.read_to_end(&mut input_data).unwrap();

    if args.flag_decompress {
        compressor.decompress(&input_data, &mut output_data).unwrap();
    } else {
        compressor.compress(&input_data, &mut output_data).unwrap();
    }

    output.write_all(&output_data).unwrap();
}
