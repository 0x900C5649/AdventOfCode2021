use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about="Advent of Code 2021 - Day 11", version="0.1", author = "Joshua Steffensky<joshua@steffensky.io>")]
pub struct Args {
    pub part: u8,
    //#[clap(short, long)]
    pub input: String,
}
