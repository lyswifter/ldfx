use flate2::read::GzDecoder;
use std::io::{Cursor, Result};
use std::vec::Vec;
use tar::Archive;

pub fn wallet_canister() -> Result<Archive<GzDecoder<Cursor<Vec<u8>>>>> {
    let mut v = Vec::new();
    v.extend_from_slice(include_bytes!("../../distributed/wallet_canister.tgz"));

    let tar = GzDecoder::new(std::io::Cursor::new(v));
    let archive = Archive::new(tar);
    Ok(archive)
}