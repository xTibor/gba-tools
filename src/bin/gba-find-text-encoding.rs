extern crate docopt;
extern crate gba_rs;
extern crate gba_tools;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use gba_tools::format_offset;
use gba_tools::streams::{InputStream, OutputStream};
use std::io::{Read, Write};

const USAGE: &'static str = "
Usage:
    gba-find-text-encoding --string <string> [--input <input>] [--output <output>] [--hex]
    gba-find-text-encoding --help

Options:
    -s, --string <string>  String to search for
    -i, --input <input>    Input file
    -o, --output <output>  Output file
    -H, --hex              Use hexadecimal offsets
    -h, --help             Display this message
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_string: String,
    flag_input: Option<String>,
    flag_output: Option<String>,
    flag_hex: bool,
}

fn diff_u8(buf: &[u8]) -> Vec<isize> {
    buf.windows(2)
       .map(|window| window[1] as isize - window[0] as isize)
       .collect::<Vec<isize>>()
}

fn diff_u16le(buf: &[u8]) -> Vec<isize> {
    Vec::new()
}

fn diff_u16be(buf: &[u8]) -> Vec<isize> {
    Vec::new()
}

fn diff_u16lo(buf: &[u8]) -> Vec<isize> {
    Vec::new()
}

fn diff_u16hi(buf: &[u8]) -> Vec<isize> {
    Vec::new()
}

fn diff_u32le(buf: &[u8]) -> Vec<isize> {
    Vec::new()
}

fn diff_u32be(buf: &[u8]) -> Vec<isize> {
    Vec::new()
}

fn find_text(needle_diffs: &[isize], haystack_diffs: &[isize], output: &mut OutputStream, name: &'static str) {
    for (offset, window) in haystack_diffs.windows(needle_diffs.len()).enumerate() {
        if *window == needle_diffs[..] {
            writeln!(output, "{}", name).unwrap();
        }
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_string.len() < 2 {
        eprintln!("gba-find-text-encoding: `--string` must be at least 2 characters long");
        return
    }

    let mut input = InputStream::new(args.flag_input).unwrap();
    let mut output = OutputStream::new(args.flag_output).unwrap();

    let mut input_data = Vec::new();
    input.read_to_end(&mut input_data).unwrap();

    let needle_diffs = diff_u8(args.flag_string.as_bytes());

    find_text(&needle_diffs, &diff_u8(&input_data), &mut output, "u8");
    find_text(&needle_diffs, &diff_u16le(&input_data), &mut output, "u16le");
    find_text(&needle_diffs, &diff_u16be(&input_data), &mut output, "u16be");
    find_text(&needle_diffs, &diff_u16lo(&input_data), &mut output, "u16lo");
    find_text(&needle_diffs, &diff_u16hi(&input_data), &mut output, "u16hi");
    find_text(&needle_diffs, &diff_u32le(&input_data), &mut output, "u32le");
    find_text(&needle_diffs, &diff_u32be(&input_data), &mut output, "u32be");
}
