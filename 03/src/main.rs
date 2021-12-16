//siblings
//std
use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::*;

//dependencies
use clap::Parser;
use itertools::zip;

#[derive(Parser, Debug)]
#[clap(
    about = "Advent of Code 2021 - Day 03",
    version = "0.1",
    author = "Joshua Steffensky<joshua@steffensky.dev>"
)]
struct Args {
    part: u8,
    //#[clap(short, long)]
    input: String,
}

fn main() {
    let args: Args = Args::parse();

    match args.part {
        1 => {
            part1(args);
        }
        2 => {
            part2(args);
        }
        _ => {
            println!("unknown part: \"{}\"", args.part);
        }
    }
}

fn part1(args: Args) -> (u64, u64) {
    println!("Part1:");
    let mut histgram: Vec<u64> = Vec::new();
    let mut count = 0;
    let mut gammastr: String = "".to_string();
    let mut epsilonstr: String = "".to_string();
    let mut gamma = 0;
    let mut epsilon = 0;
    let file = File::open(args.input).unwrap();
    if let lines = io::BufReader::new(file).lines() {
        // consumes the iterator, returns an (optional) string
        for line in lines {
            if let Ok(linestring) = line {
                //parse current line
                for (c, idx) in zip(linestring.chars(), (0..linestring.chars().count())) {
                    if c == '1' {
                        if histgram.len() == 0 {
                            for _ in (0..linestring.chars().count()) {
                                histgram.push(0);
                            }
                        }
                        histgram[idx] += 1;
                    }
                }
                count += 1;
            }
        }
    }

    for idx in (0..histgram.len()) {
        gammastr.push(if histgram[idx] > count / 2 { '1' } else { '0' });
        epsilonstr.push(if histgram[idx] < count / 2 { '1' } else { '0' });
    }

    gamma = u64::from_str_radix(&gammastr, 2).unwrap();
    epsilon = u64::from_str_radix(&epsilonstr, 2).unwrap();
    println!("\t histogram: {:?}", histgram);
    println!("\t Gamma:{} Epsilon:{}", gamma, epsilon);
    println!("\t mult: {}", gamma * epsilon);

    return (gamma, epsilon);
}

fn part2(args: Args) -> (u64, u64) {
    //let mut course = Vec::new();
    println!("Part2:");
    let mut numbers: Vec<i64> = Vec::new();
    let mut msb = 0;
    let mut ox;
    let mut co2;

    let file = File::open(args.input).unwrap();
    if let lines = io::BufReader::new(file).lines() {
        // consumes the iterator, returns an (optional) string
        for line in lines {
            if let Ok(linestring) = line {
                //parse current line
                numbers.push(i64::from_str_radix(&linestring, 2).unwrap());
                msb = linestring.chars().count();
            }
        }
    }

    let mut ox_cand = numbers.clone();
    let mut filteridx = msb;
    while ox_cand.len() > 1 && filteridx > 0 {
        let decider = min(1, max(0,ox_cand.iter().fold(1 as i64,
                                    |acc, &x| 
                                        acc + (x>>(filteridx-1) & 1) 
                                            + (x>>(filteridx-1) & 1) -1 
                                       )
                                ));
        ox_cand.retain(|&x| (x>>(filteridx -1) & 1 == decider));
        filteridx -= 1;
    }
    #[cfg(test)]
    println!("ox len: {}", ox_cand.len());
    ox = ox_cand[0];

    let mut co2_cand = numbers.clone();
    filteridx = msb;
    while co2_cand.len() > 1 && filteridx > 0{
        let decider = min(1, max(0,co2_cand.iter().fold(1 as i64,
                                    |acc, &x| 
                                        acc + (x>>(filteridx-1) & 1) 
                                            + (x>>(filteridx-1) & 1) -1 
                                       )
                                ));
        co2_cand.retain(|&x| (x>>(filteridx -1) & 1 != decider));
        filteridx -= 1;
    }
    #[cfg(test)]
    println!("co2 len: {}", co2_cand.len());
    co2 = co2_cand[0];

    println!("Ox: {}, CO2: {}", ox, co2);
    println!("mult: {}", ox*co2);
    return (ox as u64, co2 as u64);
}

#[cfg(test)]
mod test {
    #[test]
    fn p1() {
        let arg = crate::Args {
            part: 1,
            input: "input/test-measures.txt".to_string(),
        };
        let (g, e) = crate::part1(arg);
        assert_eq!(g, 22);
        assert_eq!(e, 9);
    }

    #[test]
    fn p2() {
        let arg = crate::Args {
            part: 1,
            input: "input/test-measures.txt".to_string(),
        };
        let (ox, co2) = crate::part2(arg);
        assert_eq!(ox, 23);
        assert_eq!(co2, 10);
    }
}
