mod data_loader;
mod images;
mod tierlist;
pub mod downloader;
pub mod paths;
use std::io::{self, Write};
use log::{error, info};
fn print_tierlist(tierlist: &[Vec<String>]) {
    for (tier, items) in tierlist.iter().enumerate() {
        println!(
            "{}: {}",
            if tier == 0 { 'S' } else { (b'A' + (tier - 1) as u8) as char },
            items.join(", "),
        );
    }
}

fn console_wait() {
    print!("Press Enter to continue...");
    if io::stdout().flush().is_err() {
        return;
    }

    if io::stdin().read_line(&mut String::new()).is_err() {
        return;
    }
}

fn tierlist_creator() -> Result<(), String> {
    info!("Reading data from JSON files...");
    let data = data_loader::read_all().map_err(|e| e.to_string())?;
    let legend_tierlist = tierlist::create_legend_tierlist(data)?;
    
    match images::create_image(&legend_tierlist) {
        Ok(()) => info!("Created tierlist image"),
        Err(e) => {
            error!("Failed to create image: {}", e);
            info!("Fallback - printing tierlist to console:");
            print_tierlist(&legend_tierlist);
        }
    }
    Ok(())
}

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    if let Err(e) = tierlist_creator() {
        error!("{}", e);
    }
    console_wait();
}