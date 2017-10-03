use std::fs::{self, File};
use std::io::{self, Read, Write};

#[derive(Debug)]
pub enum InputStream {
    File(fs::File),
    Stdin(io::Stdin),
}

#[derive(Debug)]
pub enum OutputStream {
    File(fs::File),
    Stdout(io::Stdout),
}

impl InputStream {
    pub fn new(arg: Option<String>) -> Result<InputStream, &'static str> {
        match arg {
            Some(path) => Ok(InputStream::File(File::open(path).unwrap())),
            None => Ok(InputStream::Stdin(io::stdin())),
        }
    }
}

impl Read for InputStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            InputStream::File(ref mut file) => file.read(buf),
            InputStream::Stdin(ref mut stdin) => stdin.read(buf),
        }
    }
}

impl OutputStream {
    pub fn new(arg: Option<String>) -> Result<OutputStream, &'static str> {
        match arg {
            Some(path) => Ok(OutputStream::File(File::create(path).unwrap())),
            None => Ok(OutputStream::Stdout(io::stdout())),
        }
    }
}

impl Write for OutputStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            OutputStream::File(ref mut file) => file.write(buf),
            OutputStream::Stdout(ref mut stdout) => stdout.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            OutputStream::File(ref mut file) => file.flush(),
            OutputStream::Stdout(ref mut stdout) => stdout.flush(),
        }
    }
}
