extern crate docopt;
extern crate gba_compression;
extern crate gba_tools;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use gba_compression::bios::decompress_lz77;
use gba_tools::format_offset;
use gba_tools::streams::{InputStream, OutputStream};
use std::fs::File;
use std::io::{Read, Write};

const USAGE: &'static str = "
Usage:
    gba-find-lz77 [--input <input>] [--output <output>] [--min-size <bytes>] [--dump-dir <directory>] [--hex] [--silent]
    gba-find-lz77 --help

Options:
    -i, --input <input>         Input file
    -o, --output <output>       Output file
    -m, --min-size <bytes>      Discard data below this size
    -d, --dump-dir <directory>  Dump the found data into <diretory>
    -H, --hex                   Use hexadecimal offsets
    -s, --silent                Do not print the offsets to the output
    -h, --help                  Display this message
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_input: Option<String>,
    flag_output: Option<String>,
    flag_dump_dir: Option<String>,
    flag_min_size: Option<usize>,
    flag_hex: bool,
    flag_silent: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut input = InputStream::new(args.flag_input).unwrap();
    let mut output = OutputStream::new(args.flag_output).unwrap();

    let mut input_data = Vec::new();
    input.read_to_end(&mut input_data).unwrap();

    let mut offset = 0;
    while offset < input_data.len() {
        if let Ok(decompressed) = decompress_lz77(&input_data[offset..]) {
            if decompressed.len() >= args.flag_min_size.unwrap_or(0) {
                let offset_str = format_offset(offset, args.flag_hex);

                if ! args.flag_silent {
                    writeln!(output, "{}", offset_str).unwrap();
                }

                if let Some(ref dir) = args.flag_dump_dir {
                    let path = format!("{}/lz77_{}.bin", dir, offset_str);
                    let mut file = File::create(path).unwrap();
                    file.write_all(&decompressed).unwrap();
                }
            }
        }

        offset += 4;
    }
}
