mod cli;
mod conf;
mod error;
mod io_fs_errors;
mod library;

use inquire::InquireError;
use owo_colors::OwoColorize;

const LLIBRE_ASCII: &str = "\
 █████       ████   ███  █████                       
░░███       ░░███  ░░░  ░░███                        
 ░███        ░███  ████  ░███████  ████████   ██████ 
 ░███        ░███ ░░███  ░███░░███░░███░░███ ███░░███
 ░███        ░███  ░███  ░███ ░███ ░███ ░░░ ░███████ 
 ░███      █ ░███  ░███  ░███ ░███ ░███     ░███░░░  
 ███████████ █████ █████ ████████  █████    ░░██████ 
░░░░░░░░░░░ ░░░░░ ░░░░░ ░░░░░░░░  ░░░░░      ░░░░░░  ";

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("\n{}\n", LLIBRE_ASCII);
    println!("{} v{}", "Llibre".bold().underline().italic(), VERSION);

    if let Err(e) = cli::init() {
        println!("{}", e)
    }
}
