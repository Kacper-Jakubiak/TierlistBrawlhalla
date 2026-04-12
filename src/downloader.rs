use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{create_dir_all, OpenOptions};
use std::io::{copy, Error as IoError};
use std::path::PathBuf;

use log::info;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use url::Url;
use crate::paths;

#[derive(Debug)]
pub enum DownloadError {
    Online(String),
    Io(IoError),
    Path(String),
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DownloadError::Online(e) => write!(f, "Online error: {}", e),
            DownloadError::Io(e) => write!(f, "I/O error: {}", e),
            DownloadError::Path(e) => write!(f, "Path error: {}", e),
        }
    }
}

impl Error for DownloadError {}

impl From<reqwest::Error> for DownloadError {
    fn from(e: reqwest::Error) -> Self {
        DownloadError::Online(e.to_string())
    }
}

impl From<url::ParseError> for DownloadError {
    fn from(e: url::ParseError) -> Self {
        DownloadError::Online(e.to_string())
    }
}

impl From<IoError> for DownloadError {
    fn from(e: IoError) -> Self {
        DownloadError::Io(e)
    }
}


pub struct Downloader {
    client: Client,
    base_url: Url,
    document: Html,
    selector: Selector,
}

impl Downloader {
    pub fn new(page_url: &str) -> Result<Self, DownloadError> {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (compatible; RustDownloader/1.0)")
            .build()?;
        let base_url = Url::parse(page_url)?;
        let body = client.get(page_url).send()?.text()?;
        let document = Html::parse_document(&body);
        let selector = Selector::parse("img")
            .map_err(|e| DownloadError::Online(format!("Selector parse error: {}", e)))?;
        Ok(Self {
            client,
            base_url,
            document,
            selector,
        })
    }

    pub fn download(&self) -> Result<(), DownloadError> {
        let dir_path = paths::get_asset_path()
            .ok_or_else(|| DownloadError::Path("Asset path not found".to_string()))?;
        create_dir_all(dir_path.join("weapons"))?;
        create_dir_all(dir_path.join("legends"))?;

        for img in self.document.select(&self.selector) {
            if let Some(src) = img.value().attr("src") {
                let img_url = self.base_url.join(src)?;

                let file_name = get_name(&img_url)
                    .ok_or_else(|| DownloadError::Path("Failed to extract filename from URL".to_string()))?;

                if !file_name.ends_with(".png") {
                    continue;
                }
                let file_path = path_from_filename(&dir_path, &file_name)
                    .map_err(|e| DownloadError::Path(e))?;
                if file_path.exists() {
                    continue;
                }

                info!("Downloading {}", file_name);

                let mut response = self.client.get(img_url).send()?;
                let mut out_file = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&file_path)?;

                copy(&mut response, &mut out_file)?;
            }
        }
        Ok(())
    }
}

fn get_name(img_url: &Url) -> Option<&str> {
    img_url
        .path_segments()
        .and_then(|segments| segments.last())
        .filter(|f_name| !f_name.is_empty())
}

fn path_from_filename(base_dir: &PathBuf, filename: &str) -> Result<PathBuf, String> {
    let extra_path: PathBuf = if filename.ends_with("Classic.png") {
        let new_name = trim_unicode_slice(filename, 14, 11)
            .ok_or_else(|| format!("Invalid legend filename: {}", filename))?;
        PathBuf::from("legends").join(format!("{}.png", new_name))
    } else if filename.ends_with("Icon.png") {
        let new_name = trim_unicode_slice(filename, 5, 8)
            .ok_or_else(|| format!("Invalid weapon filename: {}", filename))?;
        PathBuf::from("weapons").join(format!("{}.png", new_name))
    } else {
        return Err(format!("Unexpected filename: {}", filename));
    };
    Ok(base_dir.join(extra_path))
}

fn trim_unicode_slice(s: &str, start: usize, end: usize) -> Option<&str> {
    let start_idx = s.char_indices().nth(start).map(|(i, _)| i)?;
    let end_idx = s.char_indices().rev().nth(end).map(|(i, _)| i)?;
    if start_idx >= end_idx {
        return None;
    }
    Some(&s[start_idx..end_idx])
}
