
extern crate docopt;
extern crate gba_rs;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use gba_rs::compression::bios::decompress_lz77;
use gba_rs::utils::streams::{InputStream, OutputStream};
use std::fs::File;
use std::io::{Read, Write, Cursor};

const USAGE: &'static str = "
Usage:
    gba-find-lz77 [--input <input>] [--output <output>] [--dumpdir <directory>] [--hex] [--silent]
    gba-find-lz77 --help

Options:
    -i, --input    Input file
    -o, --output   Output file
    -d, --dumpdir  Dump the found data into <diretory>
    -H, --hex      Use hexadecimal notation for offsets
    -s, --silent   Do not print the offsets to the standard output
    -h, --help     Display this message
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_input: Option<String>,
    arg_output: Option<String>,
    arg_dumpdir: Option<String>,
    flag_hex: bool,
    flag_silent: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut input = InputStream::new(args.arg_input).unwrap();
    let mut output = OutputStream::new(args.arg_output).unwrap();

    let mut input_data = Vec::new();
    input.read_to_end(&mut input_data).unwrap();

    let mut offset = 0;
    while offset < input_data.len() {
        let mut cursor = Cursor::new(&input_data[offset..]);
        let mut decompressed: Vec<u8> = Vec::new();

        if decompress_lz77(&mut cursor, &mut decompressed).is_ok() {
            let offset_str = if args.flag_hex {
                format!("{:06X}", offset)
            } else {
                format!("{}", offset)
            };

            if ! args.flag_silent {
                writeln!(output, "{}", offset_str).unwrap();
            }

            if let Some(ref dir) = args.arg_dumpdir {
                let path = format!("{}/lz77_{}.bin", dir, offset_str);
                let mut file = File::create(path).unwrap();
                file.write_all(&decompressed).unwrap();
            }
        }

        offset += 4;
    }
}
