//siblings
//std
use std::fs as fs;
//use std::fs::File;
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

    #[clap(default_value_t = 0)]
    dts: u32,
}


fn main() {
    let mut args: Args = Args::parse();
    
    match args.part
    {
        1=>
        {
            println!("Part 1:");
            args.dts=80;
            part1(args);
        },
        2=>
        {
            println!("Part 2:");
            args.dts=256;
            part1(args);
        },
        _ =>{println!("unknown part: \"{}\"", args.part);},
    }
}

//print histogram of population
fn print_pop(population: &[u64])
{
    println!("{:?}", population);
}

// returns new lanternfish on simulated day
fn p1_sim_day(population: &mut [u64]) -> u64
{
    let birthrate = population[0];
    for dtb in 0..population.len()-1 //dtb => days to birth
    {
        population[dtb] = population[dtb+1];
    }
    population[6] += birthrate;
    population[population.len()-1] = birthrate;
    //print_pop(population);
    return birthrate;
}

fn part1(args:Args) ->u64 {
    lazy_static!{
        static ref RE_POPULATION: Regex = Regex::new(r"(?P<n>\d+)(,|\s*$)").unwrap();
    }
    let mut population: [u64; 9] = [0;9];

    
    let filecontent = fs::read_to_string(args.input)
        .expect("Something went wrong reading the file");
    
    //parse population
    for cap in RE_POPULATION.captures_iter(&filecontent)
    {
        population[cap["n"].parse::<usize>().unwrap()] += 1;
    }
    println!("\t Initial population size: {}", population.iter().sum::<u64>());
    
    println!("\t Initial Population");
    print_pop(&population);

    //simulate dts days    
    #[cfg(test)]
    println!("Simulating {} days", args.dts);
    for day in 1..args.dts+1
    {
        p1_sim_day(&mut population);
        #[cfg(test)]
        println!("\tpopulation size after {} days: {} laternfish", day, population.iter().sum::<u64>());
    }
   
    println!("\t Final Population:");
    print_pop(&population);

    println!("\tpopulation size after {} days: {} laternfish", args.dts, population.iter().sum::<u64>());

    return population.iter().sum();
}

//fn part2(args:Args)->u64 {
    //println!("Part2:");
    //return part1(args);
//}

#[cfg(test)]
mod tests {
    #[test]
    fn p1() {
        let arg = crate::Args{part : 1, input : "input/test-population.txt".to_string(), dts:18};
        assert_eq!(crate::part1(arg), 26);
        
        let arg = crate::Args{part : 1, input : "input/test-population.txt".to_string(), dts:80};
        assert_eq!(crate::part1(arg), 5934);
    }

    #[test]
    fn p2(){
        let arg = crate::Args{part : 2, input : "input/test-population.txt".to_string(), dts:256};
        assert_eq!(crate::part1(arg), 26984457539);
    }
}

