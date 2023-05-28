use clap::Parser;
use std::env;
mod converter;

#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_BIN_NAME"),
    version = env!("CARGO_PKG_VERSION"),
)]
#[command(about, long_about)]
pub struct Args {
    /// The time
    #[arg(
        value_name = "time",
        help = "time to convert. if not specified, use now"
    )]
    time: Option<String>,

    /// convert format
    #[arg(short, long)]
    #[clap(value_enum, default_value_t=converter::TimeFormat::S)]
    format: converter::TimeFormat,

    /// formatted time only mode
    #[arg(short, long)]
    only_formatted: bool,
}

fn main() {
    let args = Args::parse();
    let result = converter::handle(args);
    println!("{}", result);
}
