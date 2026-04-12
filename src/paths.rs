use std::{env, path::{PathBuf}};

pub fn get_asset_path() -> Option<PathBuf> {
    if let Ok(mut exe_path) = env::current_exe() {
        exe_path.pop();
        let runtime_assets = exe_path.join("assets");
        if runtime_assets.is_dir() {
            return Some(runtime_assets);
        }
    }

    let build_assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    if build_assets.is_dir() {
        return Some(build_assets);
    }

    None
}

pub fn get_output_path() -> Option<PathBuf> {
    Some(PathBuf::from("tierlist.png"))
}