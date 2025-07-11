use crate::downloader;
use crate::paths;
use image::{DynamicImage, GenericImage, RgbaImage};
use std::collections::HashMap;
use std::fs::{create_dir_all, DirEntry};
use std::path::PathBuf;
use std::{fs, io};
use unicode_normalization::char::canonical_combining_class;
use unicode_normalization::UnicodeNormalization;

const IMG_DIM: u32 = 45;

fn check_images(names: &[String], images: &HashMap<String, DynamicImage>) -> Result<(), io::Error> {
    for name in names {
        images.get(name).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("Image not found: {}", name),
            )
        })?;
    }
    Ok(())
}

pub fn create_image(tierlist: &[Vec<String>]) -> Result<(), io::Error> {
    let (columns, rows) = get_dimensions(tierlist);
    let (grid_width, grid_height) = (columns * IMG_DIM, rows * IMG_DIM);

    let mut grid_image: RgbaImage = RgbaImage::new(grid_width, grid_height);

    let legend_path = paths::get_asset_path()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "asset path not found"))?
        .join("legends");

    if let Err(e) = create_dir_all(&legend_path) {
        return Err(io::Error::new(io::ErrorKind::InvalidData, e).into());
    }

    let mut images = load_images(&legend_path)?;
    let needed_images: Vec<String> = tierlist.iter()
        .flat_map(|vec| vec.iter())
        .map(|img_name| normalize_name(img_name))
        .collect();


    if let Err(_) = check_images(&needed_images, &images) {
        println!("Images missing, starting download...");
        download_images().map_err(|e| {
            println!("Download failed");
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;
        images = load_images(&legend_path)?;
        check_images(&needed_images, &images)?;
        println!("Download finished");
    }

    for (tier, items) in tierlist.iter().enumerate() {
        let y = tier as u32 * IMG_DIM;

        for (poz, legend) in items.iter().enumerate() {
            let x = poz as u32 * IMG_DIM;
            let filename = normalize_name(legend);

            let img = match images.get(&filename) {
                Some(img) => img,
                None => unreachable!()
            };

            grid_image.copy_from(&img.to_rgba8(), x, y).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        }
    }

    let output_path = paths::get_output_path().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "output path should be set"))?;
    if let Err(e) = fs::remove_file(&output_path) {
        if e.kind() != io::ErrorKind::NotFound {
            return Err(e.into());
        }
    }
    if let Err(e) = grid_image.save(&output_path) {
        return Err(io::Error::new(io::ErrorKind::InvalidData, e).into());
    }
    Ok(())
}


fn get_dimensions(tierlist: &[Vec<String>]) -> (u32, u32) {
    let rows = tierlist.len() as u32;
    let columns = tierlist.iter()
        .map(|t| t.len() as u32)
        .max().unwrap_or(0);
    (columns, rows)
}
fn load_images(folder_path: &PathBuf) -> Result<HashMap<String, DynamicImage>, io::Error> {
    let paths: Vec<DirEntry> = fs::read_dir(folder_path)?
        .filter_map(Result::ok)
        .collect();

    let images: HashMap<String, DynamicImage> = paths.iter()
        .filter_map(|entry| {
            Some((
                entry.file_name().into_string().ok()?,
                image::open(entry.path()).ok()?
            ))
        })
        .collect();

    Ok(images)
}

fn normalize_name(name: &str) -> String {
    name.replacen(' ', "", 1)
        .nfd()
        .filter(|c| canonical_combining_class(*c) == 0)
        .collect::<String>() + ".png"
}

fn download_images() -> Result<(), downloader::DownloadError> {
    downloader::Downloader::new("https://brawlhalla.wiki.gg/wiki/Weapons")?.download()?;
    Ok(())
}