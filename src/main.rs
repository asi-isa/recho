use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Echoes given arguments
    #[arg(required(true))]
    args: Vec<String>,

    /// Do not print new line
    #[arg(short, default_value_t = true, action = ArgAction::SetFalse)]
    newline: bool,
}

fn main() {
    let args = Args::parse();

    if !args.args.is_empty() {
        println!("there are some")
    }

    for arg in args.args.iter() {
        print!("{arg} ")
    }

    if args.newline {
        println!();
    }
}
