use flate2::{
    read::{DeflateDecoder, DeflateEncoder},
    Compression,
};
use std::io::{self, Read, Write};

/// Deflate compress
pub fn compress(bytes: &[u8]) -> io::Result<Vec<u8>> {
    let mut deflate = flate2::write::DeflateEncoder::new(Vec::new(), Compression::default());
    deflate.write_all(bytes)?;

    let finish = deflate.finish()?;
    Ok(finish)
}

/// Deflate decompress
pub fn decompress(bytes: &[u8]) -> io::Result<Vec<u8>> {
    let mut deflate = flate2::write::DeflateDecoder::new(Vec::new());
    deflate.write_all(bytes)?;

    let finish = deflate.finish()?;
    Ok(finish)
}

/// Deflate compress from io
pub fn compress_from_read<R: Read>(r: R) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    DeflateEncoder::new(r, Compression::default()).read_to_end(&mut buf)?;
    Ok(buf)
}

/// Deflate decompress from io
pub fn decompress_from_read<R: Read>(r: R) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    DeflateDecoder::new(r).read_to_end(&mut buf)?;
    Ok(buf)
}
