//siblings
//std
use std::fs as fs;
//use std::fs::File;
//use std::io::{self, BufRead};
use std::cmp::min;

//dependencies
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Parser, Debug)]
#[clap(about="Advent of Code 2021 - Day 07", version="0.1", author = "Joshua Steffensky<joshua@steffensky.dev>")]
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

/* See https://codereview.stackexchange.com/questions/173338/calculate-mean-median-and-mode-in-rust */
fn average(numbers: &[u64]) -> f32 {
    numbers.iter().sum::<u64>() as f32 / numbers.len() as f32
}

fn median(numbers: &[u64]) -> u64 {
    let mut mutn = Vec::new();
    mutn.extend_from_slice(numbers);
    mutn.sort();
    let mid = mutn.len() / 2;
    mutn[mid]
}
/* End See */

fn p1_calc_fuel(oripos: &[u64], finalposition: u64) -> u64
{
    oripos.iter().map(|&p| ((finalposition as i64)-(p as i64)).abs() as u64).sum()
}

fn p2_calc_fuel(oripos: &[u64], finalposition: u64) -> u64
{
    oripos.iter().map(|&p|((finalposition as i64)-(p as i64)).abs() as u64).map(|n| (n*(n+1)/2) as u64 ).sum()
}

fn part1(args:Args) -> u64 {
    lazy_static!{
        static ref RE_POSITIONS: Regex = Regex::new(r"(?P<n>\d+)(,|\s*$)").unwrap();
    }
    let mut positions = Vec::<u64>::new();

    
    let filecontent = fs::read_to_string(args.input)
        .expect("Something went wrong reading the file");
    
    //parse population
    for cap in RE_POSITIONS.captures_iter(&filecontent)
    {
        positions.push(cap["n"].parse::<u64>().unwrap());
    }
    println!("\tpopulation size: {}", positions.iter().count());
    
    let median = median(&positions);
    let fuelcost = p1_calc_fuel(&positions, median);
    println!("\tFuelcost for position {} is {}", median, fuelcost);
    
    return fuelcost;
}

fn part2(args:Args) -> u64 {
    lazy_static!{
        static ref RE_POSITIONS: Regex = Regex::new(r"(?P<n>\d+)(,|\s*$)").unwrap();
    }
    let mut positions = Vec::<u64>::new();

    
    let filecontent = fs::read_to_string(args.input)
        .expect("Something went wrong reading the file");
    
    //parse population
    for cap in RE_POSITIONS.captures_iter(&filecontent)
    {
        positions.push(cap["n"].parse::<u64>().unwrap());
    }
    println!("\tpopulation size: {}", positions.iter().count());
    
    let avg = average(&positions);
    let fuelcostup = p2_calc_fuel(&positions, avg.ceil() as u64);
    let fuelcostdown = p2_calc_fuel(&positions, avg.floor() as u64);
    println!("\tFuelcost @ {} : {}", avg.ceil() as u64, fuelcostup);
    println!("\tFuelcost @ {} : {}", avg.floor() as u64, fuelcostdown);
    return min(fuelcostup, fuelcostdown);
}

#[cfg(test)]
mod tests {
    #[test]
    fn p1() {
        let arg = crate::Args{part : 1, input : "input/test-positions.txt".to_string()};
        assert_eq!(crate::part1(arg), 37);
    }

    #[test]
    fn p2(){
        let arg = crate::Args{part : 2, input : "input/test-positions.txt".to_string()};
        assert_eq!(crate::part2(arg), 168);
    }
}

