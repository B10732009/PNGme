use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

mod chunk;
mod chunk_type;
mod png;

fn read_args() -> Vec<String> {
    return env::args().collect();
}

fn read_png(fname: &str) -> Result<Vec<u8>, String> {
    let pname = Path::new(fname);
    if !pname.exists() {
        return Err(String::from("[Main] File is not found."));
    }

    let mut buf: Vec<u8> = Vec::new();

    let mut f = File::open(fname).unwrap();
    let _ = f.read_to_end(&mut buf);

    return Ok(buf);
}

fn write_png(fname: &str, buf: &Vec<u8>) -> Result<(), String> {
    let mut f = File::create(fname).unwrap();
    let _ = f.write_all(buf);

    return Ok(());
}

fn encode(src_fname: &str, dst_fname: &str, chunk_type: &str, msg: &str) -> Result<(), String> {
    let buf = read_png(src_fname)?;
    let mut png = Png::from_bytes(&buf)?;

    let new_chunk = Chunk::from_str(chunk_type, msg)?;
    png.add_chunk(new_chunk);

    let new_buf = png.bytes();
    let _ = write_png(dst_fname, &new_buf)?;

    return Ok(());
}

fn decode(src_fname: &str, chunk_type: &str) -> Result<String, String> {
    let buf = read_png(src_fname)?;
    let png = Png::from_bytes(&buf)?;

    let chunk_res = png.search_chunk(chunk_type);
    match chunk_res {
        Some(chunk) => return Ok(String::from(chunk.data_str())),
        None => return Err(String::from("[Main] Chunk is not found.")),
    }
}

fn delete(src_fname: &str, chunk_type: &str) -> Result<(), String> {
    let buf = read_png(src_fname)?;
    let mut png = Png::from_bytes(&buf)?;

    let _ = png.delete_chunk(chunk_type)?;

    let new_buf = png.bytes();
    let _ = write_png(src_fname, &new_buf)?;

    return Ok(());
}

fn print(src_fname: &str) -> Result<(), String> {
    let buf = read_png(src_fname)?;
    let png = Png::from_bytes(&buf)?;
    print!("{}\n", png);

    return Ok(());
}

fn execute(args: &Vec<String>) -> Result<(), String> {
    if args[1] == "encode" && args.len() == 6 {
        return encode(&args[2], &args[3], &args[4], &args[5]);
    } else if args[1] == "decode" && args.len() == 4 {
        let res = decode(&args[2], &args[3]);
        match res {
            Ok(s) => {
                print!("Decoded Message: {}\n", s);
                return Ok(());
            }
            Err(s) => return Err(s),
        }
    } else if args[1] == "delete" && args.len() == 4 {
        return delete(&args[2], &args[3]);
    } else if args[1] == "print" && args.len() == 3 {
        return print(&args[2]);
    } else {
        return Err(String::from(
            "[Main] Invalid parameters or parameter number.",
        ));
    }
}

fn main() {
    let args = read_args();
    if args.len() < 3 {
        print!("[Main] Invalid number of arguments.\n");
        return;
    }

    let res = execute(&args);
    match res {
        Ok(()) => (),
        Err(s) => print!("{}\n", s),
    }
}
