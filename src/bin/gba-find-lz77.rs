#[macro_use]
extern crate clap;
extern crate gba_rs;

use std::fs::File;
use std::io::{Cursor, Read, Write, Result, stdout, Error, ErrorKind, Seek, SeekFrom};
use gba_rs::compression::bios::decompress_lz77;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

// TODO: Make it work with the other BIOS compressions

fn dump_single_stream(matches: &ArgMatches) -> Result<()> {
    let rom_path = matches.value_of("rom")
        .ok_or_else(|| Error::new(ErrorKind::Other, "Argument \"rom\" not found"))?;
    let offset = value_t!(matches, "offset", u64)
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    let mut rom_data = Vec::new();
    let mut rom_file = File::open(rom_path)?;
    rom_file.read_to_end(&mut rom_data)?;

    let mut input = Cursor::new(&rom_data);
    input.seek(SeekFrom::Start(offset))?;

    if let Some(output_path) = matches.value_of("output") {
        let mut output = File::create(output_path)?;
        decompress_lz77(&mut input, &mut output)
    } else {
        let stdout = stdout();
        let mut output = stdout.lock();
        decompress_lz77(&mut input, &mut output)
    }
}

fn find_all_streams<F>(matches: &ArgMatches, callback: F) -> Result<()>
    where F: Fn(&ArgMatches, &[u8], usize, usize, usize) -> Result<()>
{
    let rom_path = matches.value_of("rom")
        .ok_or_else(|| Error::new(ErrorKind::Other, "Argument \"rom\" not found"))?;
    let min_size = value_t!(matches, "min-size", usize)
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    let mut rom_data = Vec::new();
    let mut rom_file = File::open(rom_path)?;
    rom_file.read_to_end(&mut rom_data)?;

    let mut offset = 0;
    while offset < rom_data.len() {
        let mut input = Cursor::new(&rom_data[offset..]);
        let mut output: Vec<u8> = Vec::new();

        if decompress_lz77(&mut input, &mut output).is_ok() {
            if output.len() >= min_size {
                callback(matches, &output, offset, input.position() as usize, output.len())?;
            }
        }

        offset += 4;
    }

    Ok(())
}

fn callback_list(_matches: &ArgMatches, _output: &[u8], offset: usize, compressed_size: usize, decompressed_size: usize) -> Result<()> {
    println!("LZ77, Offset: {}, Size: {}/{}", offset, compressed_size, decompressed_size);
    Ok(())
}

fn callback_dump(matches: &ArgMatches, output: &[u8], offset: usize, _compressed_size: usize, _decompressed_size: usize) -> Result<()> {
    let out_dir = matches.value_of("out-dir")
        .ok_or_else(|| Error::new(ErrorKind::Other, "Argument \"out-dir\" not found"))?
        .trim_right_matches('/');
    let file_prefix = matches.value_of("file-prefix")
        .ok_or_else(|| Error::new(ErrorKind::Other, "Argument \"file-prefix\" not found"))?;

    let output_path = format!("{}/{}_{}.lz77.bin", out_dir, file_prefix, offset);
    println!("Dumping {}", output_path);

    let mut file = File::create(output_path)?;
    file.write_all(output)
}

fn main() {
    let matches = App::new("LZ77 Dumper")
        .setting(AppSettings::SubcommandRequired)
        .author("Tibor Nagy <xnagytibor@gmail.com>")
        .about("Dump LZ77 compressed data from ROM files")
        .subcommand(SubCommand::with_name("dump")
            .about("Dump data from a specified offset")
            .arg(Arg::with_name("rom")
                .long("rom")
                .value_name("ROM_FILE")
                .required(true))
            .arg(Arg::with_name("offset")
                .long("offset")
                .value_name("OFFSET")
                .required(true))
            .arg(Arg::with_name("output")
                .long("output")
                .value_name("OUTPUT_FILE")))
        .subcommand(SubCommand::with_name("dump-all")
            .about("Find and dump all compressed data")
            .arg(Arg::with_name("rom")
                .long("rom")
                .value_name("ROM_FILE")
                .required(true))
            .arg(Arg::with_name("out-dir")
                .long("out-dir")
                .value_name("DIR")
                .required(true))
            .arg(Arg::with_name("min-size")
                .long("min-size")
                .value_name("SIZE")
                .required(true)
                .default_value("32"))
            .arg(Arg::with_name("file-prefix")
                .long("file-prefix")
                .value_name("PREFIX")
                .required(true)))
        .subcommand(SubCommand::with_name("list-all")
            .about("Find and list all compressed data")
            .arg(Arg::with_name("rom")
                .long("rom")
                .value_name("ROM_FILE")
                .required(true))
            .arg(Arg::with_name("min-size")
                .long("min-size")
                .value_name("SIZE")
                .required(true)
                .default_value("32")))
        .get_matches();

    if let Err(e) = match matches.subcommand() {
         ("dump",     Some(m)) => dump_single_stream(m),
         ("dump-all", Some(m)) => find_all_streams(m, callback_dump),
         ("list-all", Some(m)) => find_all_streams(m, callback_list),
        _ => unreachable!(),
    } {
        println!("Error: {}", e);
    }
}
