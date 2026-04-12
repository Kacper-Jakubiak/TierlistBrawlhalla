mod reader;
mod images;
mod backend;
pub mod downloader;
pub mod paths;
use std::io::{self, Write};
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
    let data = reader::read_all().map_err(|e| e.to_string())?;
    let legend_tierlist = backend::create_legend_tierlist(data)
        .ok_or_else(|| "Tierlist cannot be created".to_string())?;
    
    match images::create_image(&legend_tierlist) {
        Ok(()) => println!("Created tierlist image"),
        Err(e) => {
            eprintln!("Warning: Failed to create image: {}", e);
            println!("\nFallback - printing tierlist to console:");
            print_tierlist(&legend_tierlist);
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = tierlist_creator() {
        eprintln!("Error: {}", e);
    }
    console_wait();
}