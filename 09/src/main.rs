//siblings
mod heatmap;
use heatmap::*;
use rayon::iter::IntoParallelIterator;
//std
//use std::fs as fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::rc::Rc;
//use std::cmp::min;

//dependencies
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Parser, Debug)]
#[clap(about="Advent of Code 2021 - Day 09", version="0.1", author = "Joshua Steffensky<joshua@steffensky.dev>")]
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
    let mut map: Heatmap = Heatmap::new();
    
    let file = File::open(args.input).unwrap();
    if let lines = io::BufReader::new(file).lines() {
        // consumes the iterator, returns an (optional) string
        for line in lines {
            if let Ok(linestring) = line {
                match map.parse_line(&linestring)
                {
                    Ok(_) => {},
                    Err(e) => println!("Error parsing line: {}", e),
                }

            }
        }
    }
    map.finish();
    
    let lowpoints = map.find_low_points();
    println!("\tFound the following local minima:");
    for ((x,y), value) in &lowpoints
    {
        println!("\t\t({},{}) -> {}", x,y,value);
    }
    
    let sum = lowpoints.iter().fold(0 as u64, |acc, &(_, v)| acc + v as u64 +1);
    println!("\tsum of risk levels: {}", sum);
    sum
}

fn part2(args:Args) -> u64 {
    lazy_static!{
        static ref RE_DIGITS: Regex = Regex::new(r"\|\s*(?P<d1>\w+)\s+(?P<d2>\w+)\s+(?P<d3>\w+)\s+(?P<d4>\w+)\s*$").unwrap();
    }
    let mut map: Heatmap = Heatmap::new();
    
    let file = File::open(args.input).unwrap();
    if let lines = io::BufReader::new(file).lines() {
        // consumes the iterator, returns an (optional) string
        for line in lines {
            if let Ok(linestring) = line {
                match map.parse_line(&linestring)
                {
                    Ok(_) => {},
                    Err(e) => println!("Error parsing line: {}", e),
                }

            }
        }
    }
    map.finish();
    
    map.print();
    let lowpoints = map.find_low_points();
    let refmap: Rc<Heatmap> = Rc::new(map);
    
    let mut basins : Vec<Basin> = lowpoints.into_iter()
                        .map(|p| Basin::calculate(refmap.clone(), p.0).unwrap()).collect();

    basins.sort_by(|a,b| a.size().cmp(& b.size()));

    basins.iter().for_each(|basin| println!("Basin @ {:?} with size {:?}", basin.lowpoint, basin.size()));
    
    let mult = basins[basins.len()-3..basins.len()].iter().fold(1u64, |acc, basin| acc * basin.size() as u64);
    println!("\tMult of Basin sizes: {}", mult);
    return mult;
}

#[cfg(test)]
mod tests {
    #[test]
    fn p1() {
        let arg = crate::Args{part : 1, input : "input/test-heatmap.txt".to_string()};
        assert_eq!(crate::part1(arg), 15);
    }

    #[test]
    fn p2(){
        let arg = crate::Args{part : 2, input : "input/test-heatmap.txt".to_string()};
        assert_eq!(crate::part2(arg), 1134);
    }
}


