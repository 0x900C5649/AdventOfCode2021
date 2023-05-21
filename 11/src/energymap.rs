//std
use std::collections::VecDeque;
use std::error;
use std::result::Result;
use itertools;
//3rd party
use ndarray::{Array2, Array3, ArrayView2, s, Axis};
//siblings
use crate::errors::*;

pub struct EnergyMap{
    levels: Array3<u32>,
    flashes: Vec<u32>
}

impl EnergyMap{
    pub fn new() -> EnergyMap
    {
        let mut this = EnergyMap { levels: Array3::<u32>::zeros((0,0,0)), flashes: Vec::new()};
        this.flashes.push(0);
        this
    }

    pub fn parse_initial_state(&mut self, input: &str) -> Result<(), Box<dyn error::Error>>
    {
        let (dim1, dim2, dim3) = self.levels.dim();
        if dim1 != 0 || dim2 != 0 || dim3 != 0
        {
            return Err(AlreadyInitedERR{}.into());
        }
        let mut initial:Vec<Vec<u32>> = Vec::new();
        for line in input.lines()
        {
            initial.push(Self::parse_line(line)?);
        }
        
        let init_array = Array2::from_shape_fn((initial.len(), initial[0].len()),|(x,y)| initial[y][x]);
        
        self.levels = Array3::<u32>::zeros((init_array.dim().0, init_array.dim().1, 0));
        
        self.levels.push(Axis(2), init_array.view())?;
        
        return Ok(());
    }
    
    fn parse_line(line: &str) -> Result<Vec<u32>, Box<dyn error::Error>>
    {
        let mut values:Vec<u32> = Vec::new();
        for c in line.chars()
        {
            values.push(c.to_digit(10).unwrap());
        }
        
        Ok(values)
    }
    
    pub fn print(&self)
    {
        println!("{:?}", self.levels);
    }

    pub fn print_step(&self, stepidx: usize)
    {
        let (dimx, dimy, dimz) = self.levels.dim();
//        println!("{:?}", self.levels.dim());
        println!("Step:{}", stepidx);
        if stepidx >= dimz
        {
            println!("NOT Simulated ...");
            return;
        }
        let step = self.get_step(stepidx).unwrap();

        for y in 0..dimy
        {
            for x in 0..dimx
            {
                print!("{}", step.get((x,y)).unwrap());
            }
            println!("");
        }
        println!("");
    }
    
    pub fn simulate(&mut self, rounds: u32) -> Result<u32, Box<dyn error::Error>>
    {
        let mut flashes = 0;
        for _ in 0..rounds
        {
            flashes += self.step()?;
        }
        Ok(flashes)
    }

    fn step(&mut self) -> Result<u32, Box<dyn error::Error>>
    {
        let mut flashes:Vec<(usize,usize)> = Vec::new();
        //get current dimensions
        let (dimy, dimx, dimz) = self.levels.dim();
        //get latest state
        let lstep = self.get_last_step()?;
        let mut xstep:Array2<u32>;
        xstep = Array2::from(lstep.to_owned());

        //1) First, the energy level of each octopus increases by 1
        xstep.mapv_inplace(|x| x+1);

        //2) Then, any octopus with an energy level greater than 9 flashes.
        //This increases the energy level of all adjacent octopuses by 1,
        //including octopuses that are diagonally adjacent. If this causes
        //an octopus to have an energy level greater than 9, it also flashes.
        //This process continues as long as new octopuses keep having their
        //energy level increased beyond 9.
        //(An octopus can only flash at most once per step.)
        let mut q:VecDeque::<(usize, usize)> = VecDeque::new();
        
        // intiallize queue with all cells
        itertools::iproduct!(0..dimy, 0..dimx).into_iter().for_each(|cell| {
            q.push_back(cell)
        });

        while !q.is_empty()
        {
            let coords = q.pop_front().unwrap();
            let value = xstep.get(coords).unwrap();
            if !flashes.contains(&coords) && *value > 9
            {
                flashes.push(coords);
                for modifier in itertools::iproduct!(-1..2, -1..2)
                {
                    let candidate:(i64,i64) = (coords.0 as i64 + modifier.0, coords.1 as i64 + modifier.1);
                    if    ( 0 <= candidate.0 && candidate.0 < dimy as i64) 
                       && ( 0 <= candidate.1 && candidate.1 < dimx as i64)
                    {
                        let mut adjacent = xstep.get_mut((candidate.0 as usize,candidate.1 as usize)).unwrap();
                        *adjacent += 1;
                        if !q.contains(&(candidate.0 as usize,candidate.1 as usize))
                        {
                            q.push_front((candidate.0 as usize,candidate.1 as usize));
                        }
                    }
                }
            }
        }

        //3) Finally, any octopus that flashed during this step has its energy
        //level set to 0, as it used all of its energy to flash.
        xstep.mapv_inplace(|energy| if energy>9 {0} else {energy});
        
        //commit
        self.flashes.push(flashes.len() as u32);
        let _ = self.levels.push(Axis(2), xstep.view())?;

        assert_eq!(self.levels.len_of(Axis(2)), dimz +1);

        Ok(flashes.len() as u32)
    }


    pub fn count_flashes(&self, upto: usize) -> Result<u32, Box<dyn error::Error>>
    {
        let sum = self.flashes[1..upto].iter().sum();
        
        return Ok(sum);
    }

    pub fn get_step(&self, stepidx: usize) -> Result<ArrayView2<u32>, Box<dyn error::Error>>
    {
        if self.levels.len_of(Axis(2)) < stepidx
        {
            return Err(UnsimulatedError{wanted:stepidx, simulated: self.levels.len_of(Axis(2))}.into());
        }
        Ok(self.levels.slice(s![..,..,stepidx]))
    }

    pub fn get_last_step(&self) -> Result<ArrayView2<u32>, Box<dyn error::Error>>
    {
        return self.get_step(self.levels.len_of(Axis(2))-1)
    }

    pub fn find_sync_flash(&self) -> Option<usize>
    {
        let (dim1, dim2, _) = self.levels.dim();
        let target = (dim1 * dim2) as u32;

        self.flashes.iter().position(|&f| f==target)
    }

}
