use std::env;

use chrono::Local;
use clap::Parser;
mod converter;

#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_BIN_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
#[command(author, version, about, long_about = None)]
struct Args {
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
}

fn main() {
    let args = Args::parse();

    let is_time_empty = args.time.is_none();

    let time = if is_time_empty {
        converter::iso8601_simplified::format(Local::now())
    } else {
        args.time.unwrap()
    };

    let label: &str = if is_time_empty { "now" } else { "" };

    let formatted = converter::handle(time.clone(), args.format);
    println!("{} {}-> {}", time, label, formatted);
}
