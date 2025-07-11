use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Error as IoError, ErrorKind};
use std::path::PathBuf;
use crate::paths;

pub fn read_legends() -> Result<HashMap<String, (String, String)>, IoError> {
    let path = paths::get_asset_path().unwrap().join("legends.json");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).map_err(|e| json_error_to_io(e, &path))
}

pub fn read_tierlist() -> Result<Vec<Vec<String>>, IoError> {
    let path = paths::get_asset_path().unwrap().join("tierlist.json");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).map_err(|e| json_error_to_io(e, &path))
}

fn json_error_to_io(e: serde_json::Error, path: &PathBuf) -> IoError {
    IoError::new(ErrorKind::InvalidData, format!("Failed to parse JSON file '{:?}': {}", path, e))
}

pub fn read_all() -> Result<(HashMap<String, (String, String)>, Vec<Vec<String>>), IoError> {
    let legend_map = read_legends()?;
    let weapons_tierlist = read_tierlist()?;
    Ok((legend_map, weapons_tierlist))
}