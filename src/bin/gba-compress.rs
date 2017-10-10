extern crate docopt;
extern crate gba_compression;
extern crate gba_tools;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use gba_compression::bios::{filter_diff8, unfilter_diff8};
use gba_compression::bios::{filter_diff16, unfilter_diff16};
use gba_compression::bios::{compress_huffman, decompress_huffman};
use gba_compression::bios::{compress_lz77, decompress_lz77};
use gba_compression::bios::{compress_rle, decompress_rle};
use gba_compression::game_specific::wario_land_4::{compress_wl4_rle8, decompress_wl4_rle8};
use gba_compression::game_specific::wario_land_4::{compress_wl4_rle16, decompress_wl4_rle16};
use gba_compression::game_specific::wario_land_4::{compress_wl4_rle, decompress_wl4_rle};
use gba_tools::streams::{InputStream, OutputStream};
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
    Diff16,
    Diff8,
    Huffman,
    Lz77,
    Lz77Vram,
    Rle,
    Wl4Rle,
    Wl4Rle16,
    Wl4Rle8,
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
    let mut input_data = Vec::new();
    input.read_to_end(&mut input_data).unwrap();

    let output_data = if args.flag_decompress {
        use CompressionType::*;

        match args.flag_type {
            Diff16   => unfilter_diff16(&input_data),
            Diff8    => unfilter_diff8(&input_data),
            Huffman  => decompress_huffman(&input_data),
            Lz77     => decompress_lz77(&input_data),
            Lz77Vram => decompress_lz77(&input_data),
            Rle      => decompress_rle(&input_data),
            Wl4Rle   => decompress_wl4_rle(&input_data),
            Wl4Rle16 => decompress_wl4_rle16(&input_data),
            Wl4Rle8  => decompress_wl4_rle8(&input_data),
        }
    } else {
        use CompressionType::*;

        match args.flag_type {
            Diff16   => filter_diff16(&input_data),
            Diff8    => filter_diff8(&input_data),
            Huffman  => compress_huffman(&input_data),
            Lz77     => compress_lz77(&input_data, false),
            Lz77Vram => compress_lz77(&input_data, true),
            Rle      => compress_rle(&input_data),
            Wl4Rle   => compress_wl4_rle(&input_data),
            Wl4Rle16 => compress_wl4_rle16(&input_data),
            Wl4Rle8  => compress_wl4_rle8(&input_data),
        }
    }.unwrap();

    let mut output = OutputStream::new(args.flag_output).unwrap();
    output.write_all(&output_data).unwrap();
}
