//siblings
mod submarine;
use submarine::*;
//std
use std::fs::File;
use std::io::{self, BufRead};

//dependencies
use clap::Parser;


#[derive(Parser, Debug)]
#[clap(about="Advent of Code 2021 - Day 02", version="0.1", author = "Joshua Steffensky<joshua@steffensky.dev>")]
struct Args {
    part: u8,
    //#[clap(short, long)]
    input: String,
}


fn main() {
    let args: Args = Args::parse();
    
    match args.part
    {
        1=>part1(args),
        2=>part2(args),
        _ =>println!("unknown part: \"{}\"", args.part),
    }
}


fn part1(args:Args) {
    let mut sub = Submarine::new();
    //let mut course = Vec::new();
    
    println!("Part1:");
    
    let file = File::open(args.input).unwrap();
    if let lines =io::BufReader::new(file).lines() {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(commandstr) = line {
                //parse current line
                if let Some(cmd) = Command::parse(&commandstr){
                    sub.execute_p1(cmd);
                }
            }
        }
    }

    println!("\t Sub @ p:{} d:{}",sub.position, sub.depth);
    println!("\t mult: {}", sub.position * sub.depth);
}

fn part2(args:Args) {
    let mut sub = Submarine::new();
    //let mut course = Vec::new();
    
    println!("Part2:");
    
    let file = File::open(args.input).unwrap();
    if let lines =io::BufReader::new(file).lines() {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(commandstr) = line {
                //parse current line
                if let Some(cmd) = Command::parse(&commandstr){
                    sub.execute_p2(cmd);
                }
            }
        }
    }

    println!("\t Sub @ p:{} d:{} a:{}",sub.position, sub.depth, sub.aim);
    println!("\t mult: {}", sub.position * sub.depth);
}
