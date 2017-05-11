use std::fs::File;
use std::io::{Cursor, Read, Write, Result, stdout, Error, ErrorKind, Seek, SeekFrom};
use std::char;

// A tool to guess a game's character encoding
// Expects a text string used in the game, outputs the possible encoding tables.
// Very work in progress.

// TODO: Cleanup
// TODO: Explain how it works (relative letter distances)
// TODO: Check the following formats: u8, u16_le, u16_be, u16_hi_byte, u16_lo_byte
// TODO: Command-line interface
// TODO: Export the possible encodings in a standard format
// TODO: Handle capital letters
// TODO: Group same encodings
// TODO: Write tests

fn find_text_encoding() -> Result<()> {
    let mut rom_data: Vec<u8> = Vec::new();
    let mut rom_file = File::open("rom/wario_land_4.gba")?;
    rom_file.read_to_end(&mut rom_data)?;

    let mut rom_diff: Vec<u8> = Vec::new();
    rom_diff.push(0);
    for i in 1..rom_data.len() {
        rom_diff.push(rom_data[i].wrapping_sub(rom_data[i - 1]));
    }

    let searchstring = "ieroglyphs";
    let searchstring_bytes: Vec<u8> = searchstring.as_bytes().to_vec();
    let mut searchstring_diff: Vec<u8> = Vec::new();
    for i in 1..searchstring_bytes.len() {
        searchstring_diff.push(searchstring_bytes[i].wrapping_sub(searchstring_bytes[i - 1]));
    }

    for (offset, w) in rom_diff.windows(searchstring_diff.len()).enumerate() {
        if *w == *&searchstring_diff[..] {
            println!("Found at offset 0x{:08X}", offset);
            println!("Possible text encoding:");
            for char_index in 0..26 {
                let c = char::from_u32(0x61 + char_index).unwrap();
                let b = rom_data[offset-1].wrapping_sub(searchstring_bytes[0].wrapping_sub(0x61)).wrapping_add(char_index as u8);

                println!("\"{}\" => 0x{:02X}", c, b);
            }
            println!();
        }
    }

    Ok(())
}

fn main() {
    find_text_encoding().unwrap();
}
