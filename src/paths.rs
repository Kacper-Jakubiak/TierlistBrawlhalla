use std::{env, path::{PathBuf}};

// pub fn get_asset_path() -> Option<PathBuf> {
//     let path = PathBuf::from("assets");
//     path.exists().then_some(path)
// }

pub fn get_asset_path() -> Option<PathBuf> {
    // 1. Next to the compiled executable
    if let Ok(mut exe_path) = env::current_exe() {
        exe_path.pop(); // remove the executable file name
        let runtime_assets = exe_path.join("assets");
        if runtime_assets.is_dir() {
            return Some(runtime_assets);
        }
    }

    // 2. Inside the Cargo project root
    let build_assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    if build_assets.is_dir() {
        return Some(build_assets);
    }

    // 3. Nothing found
    None
}

pub fn get_output_path() -> Option<PathBuf> {
    Some(PathBuf::from("tierlist.png"))
}