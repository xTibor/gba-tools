extern crate docopt;
extern crate gba_rs;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use gba_rs::compression::bios::{compress_rle, compress_lz77};
use gba_rs::compression::bios::{decompress_rle, decompress_lz77};
use gba_rs::compression::game_specific::wario_land_4::{compress_wl4_rle8, compress_wl4_rle16};
use gba_rs::compression::game_specific::wario_land_4::{decompress_wl4_rle};
use gba_rs::utils::streams::{InputStream, OutputStream};

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
    Lz77,
    Rle,
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

    match (args.flag_type, args.flag_decompress) {
        // Compression
        (CompressionType::Lz77, false) => compress_lz77(&mut input, &mut output).unwrap(),
        (CompressionType::Rle, false) => compress_rle(&mut input, &mut output).unwrap(),
        (CompressionType::Wl4Rle8, false) => compress_wl4_rle8(&mut input, &mut output).unwrap(),
        (CompressionType::Wl4Rle16, false) => compress_wl4_rle16(&mut input, &mut output).unwrap(),
        // Decompression
        (CompressionType::Lz77, true) => decompress_lz77(&mut input, &mut output).unwrap(),
        (CompressionType::Rle, true) => decompress_rle(&mut input, &mut output).unwrap(),
        (CompressionType::Wl4Rle8, true) => decompress_wl4_rle(&mut input, &mut output).unwrap(),
        (CompressionType::Wl4Rle16, true) => decompress_wl4_rle(&mut input, &mut output).unwrap(),
        // Unsupported
        _ => {
            let cmd_str = if args.flag_decompress { "decompression" } else { "compression" };
            eprintln!("gba-compress: {:?} {} is not supported", args.flag_type, cmd_str);
        }
    }
}
