extern crate docopt;
extern crate gba_rs;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use gba_rs::compression::bios::{compress_rle, compress_lz77};
use gba_rs::compression::game_specific::wario_land_4::{compress_wl4_rle8, compress_wl4_rle16};
use gba_rs::utils::streams::{InputStream, OutputStream};

const USAGE: &'static str = "
Usage:
    gba_compress --type <type> [--input <input>] [--output <output>]
    gba_compress --help

Options:
    -i, --input   Input file
    -o, --output  Place the output into <output>
    -t, --type    Compression type
    -h, --help    Display this message
";

#[derive(Debug, Deserialize)]
enum CompressionType {
    Lz77,
    Rle,
    Wl4Rle8,
    Wl4Rle16,
}

#[derive(Debug, Deserialize)]
struct Args {
    arg_type: CompressionType,
    arg_input: Option<String>,
    arg_output: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut input = InputStream::new(args.arg_input).unwrap();
    let mut output = OutputStream::new(args.arg_output).unwrap();

    match args.arg_type {
        CompressionType::Lz77 => compress_lz77(&mut input, &mut output).unwrap(),
        CompressionType::Rle => compress_rle(&mut input, &mut output).unwrap(),
        CompressionType::Wl4Rle8 => compress_wl4_rle8(&mut input, &mut output).unwrap(),
        CompressionType::Wl4Rle16 => compress_wl4_rle16(&mut input, &mut output).unwrap(),
    }
}
