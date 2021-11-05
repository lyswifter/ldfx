use crate::lib::error::DfxResult;
use slog::info;
use std::io::Read;

use crate::util::load_assets::*;

use flate2::read::GzDecoder;
use std::io::{Cursor, Result};
use std::vec::Vec;
use tar::Archive;

// include!(concat!(env!("OUT_DIR"), "/load_assets.rs"));

pub fn dfinity_logo() -> String {
    if atty::is(atty::Stream::Stdout) {
        include_str!("../assets/dfinity-color.aart").to_string()
    } else {
        include_str!("../assets/dfinity-nocolor.aart").to_string()
    }
}

pub fn wallet_wasm(logger: &slog::Logger) -> DfxResult<Vec<u8>> {
    let mut wasm = Vec::new();

    if let Ok(dfx_wallet_wasm) = std::env::var("DFX_WALLET_WASM") {
        info!(logger, "Using wasm at path: {}", dfx_wallet_wasm);
        std::fs::File::open(&dfx_wallet_wasm)?.read_to_end(&mut wasm)?;
    } else {
        let mut canister_assets = wallet_canister()?;
        for file in canister_assets.entries()? {
            let mut file = file?;
            if file.header().path()?.ends_with("wallet.wasm") {
                file.read_to_end(&mut wasm)?;
            }
        }
    }
    Ok(wasm)
}

// new_project_files
pub fn new_project_files() -> Result<Archive<GzDecoder<Cursor<Vec<u8>>>>> {
    let mut v = Vec::new();
    v.extend_from_slice(include_bytes!("../../distributed/new_project_files.tgz"));

    let tar = GzDecoder::new(std::io::Cursor::new(v));
    let archive = Archive::new(tar);
    Ok(archive)
}

// new_project_node_files
pub fn new_project_node_files() -> Result<Archive<GzDecoder<Cursor<Vec<u8>>>>> {
    let mut v = Vec::new();
    v.extend_from_slice(include_bytes!("../../distributed/new_project_node_files.tgz"));

    let tar = GzDecoder::new(std::io::Cursor::new(v));
    let archive = Archive::new(tar);
    Ok(archive)
}

// binary_cache
pub fn binary_cache() -> Result<Archive<GzDecoder<Cursor<Vec<u8>>>>> {
    let mut v = Vec::new();
    v.extend_from_slice(include_bytes!("../../distributed/binary_cache.tgz"));

    let tar = GzDecoder::new(std::io::Cursor::new(v));
    let archive = Archive::new(tar);
    Ok(archive)
}

// assetstorage_canister
pub fn assetstorage_canister() -> Result<Archive<GzDecoder<Cursor<Vec<u8>>>>> {
    let mut v = Vec::new();
    v.extend_from_slice(include_bytes!("../../distributed/assetstorage_canister.tgz"));

    let tar = GzDecoder::new(std::io::Cursor::new(v));
    let archive = Archive::new(tar);
    Ok(archive)
}

// ui_canister
pub fn ui_canister() -> Result<Archive<GzDecoder<Cursor<Vec<u8>>>>> {
    let mut v = Vec::new();
    v.extend_from_slice(include_bytes!("../../distributed/ui_canister.tgz"));

    let tar = GzDecoder::new(std::io::Cursor::new(v));
    let archive = Archive::new(tar);
    Ok(archive)
}

// language_bindings
pub fn language_bindings() -> Result<Archive<GzDecoder<Cursor<Vec<u8>>>>> {
    let mut v = Vec::new();
    v.extend_from_slice(include_bytes!("../../distributed/language_bindings.tgz"));

    let tar = GzDecoder::new(std::io::Cursor::new(v));
    let archive = Archive::new(tar);
    Ok(archive)
}
