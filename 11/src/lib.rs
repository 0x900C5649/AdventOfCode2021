use std::fs;

mod energymap;
use crate::energymap::EnergyMap;
mod errors;
mod args;
pub use args::Args;

pub fn part1(args: Args) -> u32
{
    let mut map = EnergyMap::new();
    let contents = fs::read_to_string(&args.input)
        .expect(format!("Unable to read input path: {}", args.input).as_str());

    map.parse_initial_state(&contents).expect(format!("Unable to parse initial state from: {}", args.input).as_str());
    let res = map.simulate(100).expect(format!("Unable to simulate steps").as_str());
    for stepidx in 0..10
    {
        map.print_step(stepidx);
    }
    for stepidx in 1..11
    {
        map.print_step(stepidx *10);
    }
    println!("");
    println!("=============================");
    println!("Flashes after 100 steps: {}", res);
    res
}

pub fn part2(args: Args) -> usize
{
    let mut map = EnergyMap::new();
    let contents = fs::read_to_string(&args.input)
        .expect(format!("Unable to read input path: {}", args.input).as_str());

    map.parse_initial_state(&contents).expect(format!("Unable to parse initial state from: {}", args.input).as_str());
    let mut synced_flash_step = 0;

    while synced_flash_step == 0
    {
        let _ = map.simulate(10).expect(format!("Unable to simulate steps").as_str());
        synced_flash_step = map.find_sync_flash().unwrap_or(0);
    }
    
    map.print_step(synced_flash_step-1);
    map.print_step(synced_flash_step);

    synced_flash_step
}
