// dependencies
use clap::Parser;
use AoC_11::*;

pub fn main() {
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
