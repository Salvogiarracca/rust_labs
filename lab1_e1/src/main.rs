use clap::Parser;

use crate::slugify::slugify;

mod slugify;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    string: String,
}

fn main() {
    let args = Args::parse();

    println!("Original string: {}", args.string);
    println!("Slug : {}", slugify(&args.string))
}
