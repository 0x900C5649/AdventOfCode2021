//siblings
mod Bingo;
use Bingo::*;
//std
use std::fs as fs;
//use std::io::{self, BufRead};

//dependencies
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;


#[derive(Parser, Debug)]
#[clap(about="Advent of Code 2021 - Day 04", version="0.1", author = "Joshua Steffensky<joshua@steffensky.dev>")]
struct Args {
    part: u8,
    //#[clap(short, long)]
    input: String,
}


fn main() {
    let args: Args = Args::parse();
    
    match args.part
    {
        1=>{part1(args);},
        2=>{part2(args);},
        _ =>{println!("unknown part: \"{}\"", args.part);},
    }
}


fn part1(args:Args) ->u64 {
    lazy_static!{
        static ref RE_DRAWS: Regex = Regex::new(r"(?P<n>\d+)(,|$)").unwrap();
    }
    let mut draws = Vec::new();
    let mut boards = Vec::new();
    let mut bestRound :u64 = <u64>::MAX;
    let mut bestScore :u64= 0;
    
    println!("Part1:");
    
    let filecontent = fs::read_to_string(args.input)
        .expect("Something went wrong reading the file");
    let mut iter = filecontent.split("\n\n");
    
    //parse draws
    for cap in RE_DRAWS.captures_iter(iter.next().unwrap())
    {
        draws.push(cap["n"].parse::<u8>().unwrap());
    }

    //parse & collect all boards
    for b in iter
    {
        if let Some(s) = Sheet::parse(b)
        {
            boards.push(s);
        }
    }
    
    #[cfg(test)]
    println!("found {} sheets", boards.len());
    //get Scores
    for s in boards
    {
        if let(Some(winround),sheetscore) = s.getScore(&draws)
        {
            if winround < bestRound
            {
                bestRound = winround;
                bestScore = sheetscore;
            }
        }
    }
    
    println!("Best score at round {} (draw {}) with final score {}", bestRound, draws[bestRound as usize], bestScore);
    return bestScore;
}

fn part2(args:Args)->u64 {
    lazy_static!{
        static ref RE_DRAWS: Regex = Regex::new(r"(?P<n>\d+)(,|$)").unwrap();
    }
    let mut draws = Vec::new();
    let mut boards = Vec::new();
    let mut worstRound :u64 = 0;
    let mut worstScore :u64 = 0;
    
    println!("Part1:");
    
    let filecontent = fs::read_to_string(args.input)
        .expect("Something went wrong reading the file");
    let mut iter = filecontent.split("\n\n");
    
    //parse draws
    for cap in RE_DRAWS.captures_iter(iter.next().unwrap())
    {
        draws.push(cap["n"].parse::<u8>().unwrap());
    }

    //parse & collect all boards
    for b in iter
    {
        if let Some(s) = Sheet::parse(b)
        {
            boards.push(s);
        }
    }
    
    #[cfg(test)]
    println!("found {} sheets", boards.len());
    //get Scores
    for s in boards
    {
        if let(Some(winround),sheetscore) = s.getScore(&draws)
        {
            if winround > worstRound
            {
                worstRound = winround;
                worstScore = sheetscore;
            }
        }
    }
    
    println!("worst score at round {} (draw {}) with final score {}", worstRound, draws[worstRound as usize], worstScore);
    return worstScore;
}

#[cfg(test)]
mod tests {
    #[test]
    fn p1() {
        let arg = crate::Args{part : 1, input : "input/test-boards.txt".to_string()};
        assert_eq!(crate::part1(arg), 4512);
    }

    #[test]
    fn p2(){
        let arg = crate::Args{part : 1, input : "input/test-boards.txt".to_string()};
        assert_eq!(crate::part2(arg), 1924);
    }
}
