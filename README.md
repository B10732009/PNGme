# PNGme : Hide Secret Messages in PNG Files #

My implementation of a small Rust project from [here](https://picklenerd.github.io/pngme_book/).

## System ##

- Use `Rust` to implement the main encoder/decoder.
- Use `Python` to generate testing PNG files.
- Use `Cargo` to manage and build the project.

## Usage ##

- To build the system
    ```bash
    cargo build --release
    ```

- To encode a message into a PNG file
    ```bash
    cargo run --release encode <src_file> <dst_file> <chunk_type> <message>
    ```

- To decode a message from a PNG file
    ```bash
    cargo run --release decode <src_file> <chunk_type>
    ```

- To delete a chunk from a PNG file
    ```bash
    cargo run --release delete <src_file> <chunk_type>
    ```

- To print the data in a PNG file
    ```
    cargo run --release print <src_file>
    ```

    - The data will be printed in `json-like` format.

      ```py
        Png: {
            Header: [137, 80, 78, 71, 13, 10, 26, 10],
            Chunks: {
                Chunk: { Length: 13, ChunkType: { Bytes: [73, 72, 68, 82]}, ChunkData: [0, 0, 0, 2, 0, 0, 0, 2, 8, 2, 0, 0, 0], Crc: 4258568819},
                Chunk: { Length: 22, ChunkType: { Bytes: [73, 68, 65, 84]}, ChunkData: [120, 156, 99, 236, 18, 255, 123, 172, 82, 154, 197, 87, 224, 232, 43, 222, 63, 0, 41, 8, 6, 19], Crc: 3998231332},
                Chunk: { Length: 0, ChunkType: { Bytes: [73, 69, 78, 68]}, ChunkData: [], Crc: 2923585666},
            }
        }
        ```

- To generate a testing PNG file
    ```bash
    python3 test/generate_png.py <file> <height> <width>
    ```

## References ##

- [PNGme: An Intermediate Rust Project](https://picklenerd.github.io/pngme_book/)
- [PNG File Structure Spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust CRC Package](https://docs.rs/crc/2.1.0/crc/struct.Crc.html)
