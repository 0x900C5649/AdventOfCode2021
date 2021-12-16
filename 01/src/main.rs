//#[macro_use]
//extern crate clap;

//std
use std::fs::File;
use std::io::{self, BufRead};

//dependencies
use clap::Parser;


#[derive(Parser, Debug)]
#[clap(about="Advent of Code 2021 - Day 01", version="0.1", author = "Joshua Steffensky<joshua@steffensky.dev>")]
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


fn part1(args:Args)
{
    let mut count: u64 = 0;
    let mut prev: u64 = u64::MAX;
    let mut current: u64 = u64::MAX;
    
    println!("Part1:");
    
    let file = File::open(args.input).unwrap();
    if let lines =io::BufReader::new(file).lines() {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(depth) = line {
                //parse current line
                if let Ok(current) = depth.parse::<u64>(){
                    //compare to prev
                    if current > prev
                    {
                        //if: count +1
                        count+=1;
                    }
                    //replace prev
                    prev = current;
                }
            }
        }
    }
    println!("\t{}",count);
}

fn part2(args:Args)
{
    let mut raw = Vec::new();
    let mut count: u64 = 0;
    let mut prev: u64 = u64::MAX;
    let mut current: u64 = u64::MAX;

    println!("Part1:");
    
    let file = File::open(args.input).unwrap();
    if let lines =io::BufReader::new(file).lines() {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(depth) = line {
                //parse current line
                if let Ok(current) = depth.parse::<u64>(){
                    raw.push(current);
                }
            }
        }
    }

    //for slice in raw.windows(3)
    for window in raw.windows(3)
    {
        //let window = slice.to_vec();
        assert_eq!(window.len(), 3);
        current = window.iter().fold(0, |acc, x| acc+x);
        if current > prev{
            count += 1;
        }
        prev = current;
    }

    println!("\t{}",count);
}
