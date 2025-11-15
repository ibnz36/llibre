mod conf;
mod consts;

use owo_colors::OwoColorize;

fn main() {
    println!("\n{}\n", consts::LLIBRE_ASCII);
    println!(
        "{} v{}",
        "Llibre".bold().underline().italic(),
        consts::VERSION
    )
}
