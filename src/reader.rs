use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Error as IoError, ErrorKind};
use std::path::PathBuf;
use crate::paths;

pub fn read_legends(asset_path: &PathBuf) -> Result<HashMap<String, (String, String)>, IoError> {
    let path = asset_path.join("legends.json");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).map_err(|e| IoError::new(ErrorKind::InvalidData, format!("Failed to parse JSON file '{:?}': {}", path, e)))
}

pub fn read_tierlist(asset_path: &PathBuf) -> Result<Vec<Vec<String>>, IoError> {
    let path = asset_path.join("tierlist.json");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).map_err(|e| IoError::new(ErrorKind::InvalidData, format!("Failed to parse JSON file '{:?}': {}", path, e)))
}

pub fn read_all() -> Result<(HashMap<String, (String, String)>, Vec<Vec<String>>), IoError> {
    let asset_path = paths::get_asset_path().ok_or_else(|| IoError::new(ErrorKind::NotFound, "Asset path not found"))?;
    let legend_map = read_legends(&asset_path)?;
    let weapons_tierlist = read_tierlist(&asset_path)?;
    Ok((legend_map, weapons_tierlist))
}