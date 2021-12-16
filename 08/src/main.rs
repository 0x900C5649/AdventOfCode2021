//siblings
mod seven;
use seven::*;
//std
//use std::fs as fs;
use std::fs::File;
use std::io::{self, BufRead};
//use std::cmp::min;

//dependencies
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Parser, Debug)]
#[clap(about="Advent of Code 2021 - Day 08", version="0.1", author = "Joshua Steffensky<joshua@steffensky.dev>")]
struct Args {
    part: u8,
    //#[clap(short, long)]
    input: String,
}


fn main() {
    let args: Args = Args::parse();
    
    match args.part
    {
        1=>
        {
            println!("Part 1:");
            part1(args);
        },
        2=>
        {
            println!("Part 2:");
            part2(args);
        },
        _ =>{println!("unknown part: \"{}\"", args.part);},
    }
}


fn part1(args:Args) -> u64 {
    lazy_static!{
        static ref RE_DIGITS: Regex = Regex::new(r"\|\s*(?P<d1>\w+)\s+(?P<d2>\w+)\s+(?P<d3>\w+)\s+(?P<d4>\w+)\s*$").unwrap();
    }
    let mut count = 0;
    
    let file = File::open(args.input).unwrap();
    if let lines = io::BufReader::new(file).lines() {
        // consumes the iterator, returns an (optional) string
        for line in lines {
            if let Ok(linestring) = line {
                if let Some(captures) = RE_DIGITS.captures(&linestring)
                {
                    let digits = vec![
                                        captures["d1"].to_string(),
                                        captures["d2"].to_string(),
                                        captures["d3"].to_string(),
                                        captures["d4"].to_string()
                                    ];
                    for d in digits
                    {
                        match d.len()
                        {
                            2 | 3 | 4 | 7 => {count +=1;},
                            _ => {},
                        } 
                    }
                }
            }
        }
    }
    
    println!("counted {} times 1,4,7, or 8", count);
    count
}

fn part2(args:Args) -> u64 {
    let mut count = 0;
    
    let file = File::open(args.input).unwrap();
    if let lines = io::BufReader::new(file).lines() {
        // consumes the iterator, returns an (optional) string
        for line in lines {
            if let Ok(linestring) = line {
                let number = resolve_wiring(&linestring);
                println!("got number: {}\n", number);
                count += number;
            }
        }
    }
    
    println!("Sum of all numbers: {}", count);
    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn p1() {
        let arg = crate::Args{part : 1, input : "input/test-display.txt".to_string()};
        assert_eq!(crate::part1(arg), 26);
    }

    #[test]
    fn p2(){
        let arg = crate::Args{part : 2, input : "input/test-display.txt".to_string()};
        assert_eq!(crate::part2(arg), 61229);
    }
}


