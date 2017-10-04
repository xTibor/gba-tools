extern crate byteorder;
extern crate docopt;
extern crate gba_tools;
#[macro_use]
extern crate serde_derive;

use byteorder::{ByteOrder, BigEndian, LittleEndian};
use docopt::Docopt;
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

fn delta<T: Into<i64> + Copy>(buf: &[T]) -> Vec<i64> {
    buf.windows(2)
       .map(|window| (window[1].into()).wrapping_sub(window[0].into()))
       .collect::<Vec<i64>>()
}

struct DeltaDef {
    delta_name: &'static str,
    delta_fn: Box<Fn(&[u8]) -> Vec<i64>>,
    data_size: usize,
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

    let needle_deltas = delta(args.flag_string.as_bytes());

    let delta_defs = vec![
        DeltaDef {
            delta_name: "u8",
            data_size: 1,
            delta_fn: Box::new(|buf| {
                delta(buf)
            }),
        },
        DeltaDef {
            delta_name: "u16le",
            data_size: 2,
            delta_fn: Box::new(|buf| {
                let mut buf16: Vec<u16> = vec![0; buf.len() / 2];
                LittleEndian::read_u16_into(buf, &mut buf16[..]);
                delta(&buf16)
            }),
        },
        DeltaDef {
            delta_name: "u16be",
            data_size: 2,
            delta_fn: Box::new(|buf| {
                let mut buf16: Vec<u16> = vec![0; buf.len() / 2];
                BigEndian::read_u16_into(buf, &mut buf16[..]);
                delta(&buf16)
            }),
        },
        DeltaDef {
            delta_name: "u32le",
            data_size: 4,
            delta_fn: Box::new(|buf| {
                let mut buf32: Vec<u32> = vec![0; buf.len() / 4];
                LittleEndian::read_u32_into(buf, &mut buf32[..]);
                delta(&buf32)
            }),
        },
        DeltaDef {
            delta_name: "u32be",
            data_size: 4,
            delta_fn: Box::new(|buf| {
                let mut buf32: Vec<u32> = vec![0; buf.len() / 4];
                BigEndian::read_u32_into(buf, &mut buf32[..]);
                delta(&buf32)
            }),
        },
    ];

    for delta_def in delta_defs {
        println!("{:?}", delta_def.delta_name);

        let haystack_deltas = (delta_def.delta_fn)(&input_data);
        for (offset, window) in haystack_deltas.windows(needle_deltas.len()).enumerate() {
            if *window == needle_deltas[..] {
                writeln!(output, "{} {}", offset, delta_def.delta_name).unwrap();
            }
        }
    }
}
