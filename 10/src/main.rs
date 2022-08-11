//siblings
mod syncheck;
use syncheck::*;
//std
//use std::fs as fs;
use std::fs::File;
use std::io::{self, BufRead};
//use std::cmp::min;

//dependencies
use clap::Parser;
use lazy_static::lazy_static;
//use regex::Regex;
use hashbrown::HashMap;

#[derive(Parser, Debug)]
#[clap(about="Advent of Code 2021 - Day 10", version="0.1", author = "Joshua Steffensky<joshua@steffensky.io>")]
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


lazy_static! {
    static ref CHARPNTS: HashMap<char, u64> = {
        let mut m = HashMap::new();
        m.insert(')', 3);
        m.insert(']', 57);
        m.insert('}', 1197);
        m.insert('>', 25137);
        m
    };
}

fn part1(args:Args) -> u64 {
    let mut pnts = 0;    
    let file = File::open(args.input).unwrap();
    if let lines = io::BufReader::new(file).lines() {
        // consumes the iterator, returns an (optional) string
        for line in lines{
            if let Ok(linestring) = line {
                match check_line(&linestring)
                {
                    Err(NavParseErr::CorruptedErr((c, _))) => pnts += CHARPNTS.get(&c).unwrap(),
                    _ => {},
                }
            }
        }
    }

    println!("\tGot {} points", pnts);
    pnts
}

fn get_points_for_suggestion(suggestion: String) -> u64
{
    let mut points = 0;
    for c in suggestion.chars()
    {
        points *= 5;
        points += match c 
            {
                ')' => 1, 
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => todo!(),
            };
    }
    points
}

fn part2(args:Args) -> u64 {
    let mut pnts : Vec<u64> = Vec::new();
    let file = File::open(args.input).unwrap();
    if let lines = io::BufReader::new(file).lines() {
        // consumes the iterator, returns an (optional) string
        for line in lines{
            if let Ok(linestring) = line {
                match check_line(&linestring)
                {
                    Err(NavParseErr::IncompleteErr(remainder)) =>
                        {
                            pnts.push(get_points_for_suggestion(get_correcting_string(*remainder)));
                        },
                    //Err(NavParseErr::CorruptedErr((c, _))) => pnts += CHARPNTS.get(&c).unwrap(),
                    _ => {},
                }
            }
        }
    }
    
    pnts.sort();
    //println!("{:?}", pnts);
    println!("\tGot middle points: {}", pnts[pnts.len()/2]);
    pnts[pnts.len()/2]
}

#[cfg(test)]
mod tests {
    #[test]
    fn p1() {
        let arg = crate::Args{part : 1, input : "input/test-code.txt".to_string()};
        assert_eq!(crate::part1(arg), 26397);
    }

    #[test]
    fn p2(){
        let arg = crate::Args{part : 2, input : "input/test-code.txt".to_string()};
        assert_eq!(crate::part2(arg), 288957);
    }
}


