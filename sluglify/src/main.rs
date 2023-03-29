use sluglify::slugify;
use clap::Parser;

/// Simple program to slugify a string
#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    phrase: String,
}


fn main() {
    let args = Args::parse();
    println!("{}", slugify(args.phrase.as_str()));
}
