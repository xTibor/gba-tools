extern crate docopt;
extern crate gba_rs;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use gba_rs::compression::bios::{compress_rle, compress_lz77};
use gba_rs::compression::game_specific::wario_land_4::{compress_wl4_rle8, compress_wl4_rle16};
use std::fs::{self, File};
use std::io::{self, Read, Write};

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

#[derive(Debug)]
enum Input {
    File(fs::File),
    Stdin(io::Stdin),
}

#[derive(Debug)]
enum Output {
    File(fs::File),
    Stdout(io::Stdout),
}

impl Input {
    fn new(arg: Option<String>) -> Result<Input, &'static str> {
        match arg {
            Some(path) => Ok(Input::File(File::create(path).unwrap())),
            None => Ok(Input::Stdin(io::stdin())),
        }
    }
}

impl Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Input::File(ref mut file) => file.read(buf),
            Input::Stdin(ref mut stdin) => stdin.read(buf),
        }
    }
}

impl Output {
    fn new(arg: Option<String>) -> Result<Output, &'static str> {
        match arg {
            Some(path) => Ok(Output::File(File::create(path).unwrap())),
            None => Ok(Output::Stdout(io::stdout())),
        }
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            Output::File(ref mut file) => file.write(buf),
            Output::Stdout(ref mut stdout) => stdout.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Output::File(ref mut file) => file.flush(),
            Output::Stdout(ref mut stdout) => stdout.flush(),
        }
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut input = Input::new(args.arg_input).unwrap();
    let mut output = Output::new(args.arg_output).unwrap();

    match args.arg_type {
        CompressionType::Lz77 => compress_lz77(&mut input, &mut output).unwrap(),
        CompressionType::Rle => compress_rle(&mut input, &mut output).unwrap(),
        CompressionType::Wl4Rle8 => compress_wl4_rle8(&mut input, &mut output).unwrap(),
        CompressionType::Wl4Rle16 => compress_wl4_rle16(&mut input, &mut output).unwrap(),
    }
}
