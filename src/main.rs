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

fn tierlist_creator() {
  let data = match reader::read_all() {
        Ok(data) => data,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let legend_tierlist = match backend::create_legend_tierlist(data) {
      Some(tierlist) => tierlist,
      None => {
        println!("Tierlist cannot be created");
        return;
      }
    };
    match images::create_image(&legend_tierlist) {
      Ok(()) => println!("Created tierlist image"),
      _ => print_tierlist(&legend_tierlist),
    }
}

fn main() {
    println!("{:?}", paths::get_asset_path());
    tierlist_creator();
    console_wait();
}