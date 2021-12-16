//siblings
mod world;
use world::*;
//std
//use std::fs as fs;
use std::fs::File;
use std::io::{self, BufRead};

//dependencies
use clap::Parser;


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
    let mut w = World::new();
    let mut count = 0;

    println!("Part1:");
    
    let file = File::open(args.input).unwrap();
    if let lines = io::BufReader::new(file).lines() {
        // consumes the iterator, returns an (optional) string
        for line in lines {
            if let Ok(linestring) = line {
                //parse current line
                if let Some(l) = Line::parse(&linestring)
                {
                    if l.is_orthogonal()
                    {
                        w.add_line(l);
                    }
                    count += 1;
                }
            }
        }
    }

    #[cfg(test)]
    println!("\tRead {} lines", count);
    
    let intersects = w.get_intersections();

    #[cfg(test)]
    println!("\tIntersects: {:?}", intersects);

    println!("\tFound {} intersections", intersects.len());

    return intersects.len() as u64;
}

fn part2(args:Args)->u64 {
    let mut w = World::new();
    let mut count = 0;

    println!("Part2:");
    
    let file = File::open(args.input).unwrap();
    if let lines = io::BufReader::new(file).lines() {
        // consumes the iterator, returns an (optional) string
        for line in lines {
            if let Ok(linestring) = line {
                //parse current line
                if let Some(l) = Line::parse(&linestring)
                {
                    w.add_line(l);
                    count += 1;
                }
            }
        }
    }

    #[cfg(test)]
    println!("\tRead {} lines", count);
    
    let intersects = w.get_intersections();

    #[cfg(test)]
    println!("\tIntersects: {:?}", intersects);

    println!("\tFound {} intersections", intersects.len());

    return intersects.len() as u64;
}

#[cfg(test)]
mod tests {
    #[test]
    fn p1() {
        let arg = crate::Args{part : 1, input : "input/test-lines.txt".to_string()};
        assert_eq!(crate::part1(arg), 5);
    }

    #[test]
    fn p2(){
        let arg = crate::Args{part : 2, input : "input/test-lines.txt".to_string()};
        assert_eq!(crate::part2(arg), 12);
    }
}

