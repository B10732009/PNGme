use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

mod chunk;
mod chunk_type;
mod png;

// pub type Error = Box<dyn std::error::Error>;
// pub type Result<T> = std::result::Result<T, Error>;

fn read_args() -> Vec<String> {
    return env::args().collect();
}

fn read_png(fname: &str) -> Result<Vec<u8>, String> {
    let pname = Path::new(fname);
    if !pname.exists() {
        return Err(String::from("File is not found."));
    }

    let mut buf: Vec<u8> = Vec::new();

    let mut f = File::open(fname).unwrap();
    let _ = f.read_to_end(&mut buf);

    return Ok(buf);
}

fn write_png(fname: &str, buf: &Vec<u8>) -> Result<(), String> {
    let pname = Path::new(fname);
    // if pname.exists() {
    //     return Err(String::from("File already exists."));
    // }

    let mut f = File::create(fname).unwrap();
    let _ = f.write_all(buf);

    return Ok(());
}

fn encode(src_fname: &str, dst_fname: &str, chunk_type: &str, msg: &str) -> Result<(), String> {
    let buf = read_png(src_fname)?;
    let mut png = Png::try_from(buf.as_slice())?;

    let new_chunk_type = ChunkType::from_str(chunk_type)?;
    let new_data = msg.as_bytes().to_vec();
    let new_chunk = Chunk::new(new_chunk_type, new_data);

    png.append_chunk(new_chunk);

    let new_buf = png.as_bytes();
    let _ = write_png(dst_fname, &new_buf)?;

    return Ok(());
}

fn decode(src_fname: &str, chunk_type: &str) -> Result<String, String> {
    let buf = read_png(src_fname)?;
    let mut png = Png::try_from(buf.as_slice())?;

    if let Some(chunk) = png.chunk_by_type(chunk_type) {
        return Ok(chunk.to_string());
    }
    return Err(String::from("Chunk not found."));
}

fn remove(src_fname: &str, chunk_type: &str) -> Result<(), String> {
    let rbuf = read_png(src_fname)?;
    let mut png = Png::try_from(rbuf.as_slice())?;

    let _ = png.remove_chunk(chunk_type)?;

    let new_buf = png.as_bytes();
    let _ = write_png(src_fname, &new_buf)?;

    return Ok(());
}

fn print(src_fname: &str) -> Result<(), String> {
    let buf = read_png(src_fname)?;
    let png = Png::try_from(buf.as_slice())?;
    print!("{}\n", png);

    return Ok(());
}

// fn execute(args: &Vec<String>) ->

fn main() {
    let args = read_args();
    if args.len() < 3 {
        print!("Invalid number of arguments.\n");
        return;
    }

    print!("{:?}\n", args);

    if args[1] == "encode" && args.len() == 6 {
        let ret = encode(&args[2], &args[3], &args[4], &args[5]);
        match ret {
            Ok(()) => (),
            Err(s) => print!("{}\n", s),
        }
    } else if args[1] == "decode" && args.len() == 4 {
        let ret = decode(&args[2], &args[3]);
        match ret {
            Ok(s) => print!("{}\n", s),
            Err(s) => print!("{}\n", s),
        }
    } else if args[1] == "remove" {
        let ret = remove(&args[2], &args[3]);
        match ret {
            Ok(()) => (),
            Err(s) => print!("{}\n", s),
        }
    } else if args[1] == "print" && args.len() == 3 {
        let ret = print(&args[2]);
        match ret {
            Ok(()) => (),
            Err(s) => print!("{}\n", s),
        }
    } else {
        eprint!("Invalid number or content of parameters.\n");
    }
}
