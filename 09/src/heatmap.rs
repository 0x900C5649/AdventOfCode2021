use ndarray::{Array2, ArrayView, s};
use std::result::Result;
use std::error;
use std::fmt;
use rayon::prelude::*;
use itertools::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct DiffDimERR
{
    wanted: usize,
    got: usize,
}

impl fmt::Display for DiffDimERR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Incompatible Dimensions: got {}, expected {}", self.got, self.wanted)
    }
}
impl error::Error for DiffDimERR {}

pub struct Heatmap
{
    world: Array2<u32>,
}

pub struct Basin
{
    pub lowpoint: (usize,usize),
    pub points: Vec<(usize, usize)>,
    map: Rc<Heatmap>,
}

impl Heatmap
{
    pub fn new() -> Heatmap
    {
        Heatmap{world: Array2::<u32>::zeros((0,0))}
    }

    pub fn parse_line(&mut self, input: &str) -> Result<(),Box<dyn error::Error>>
    {
        //parse each char and insert into temp vector
        let mut row:Vec<u32> = input.chars()
                                   .map(|c| c.to_digit(10))
                                   .filter(|x| x.is_some())
                                   .map(|x| x.unwrap())
                                   .collect();
        
        let (dimy, dimx) = self.world.dim();
        if dimx == 0
        {
            //println!("init world array");
            self.world = Array2::<u32>::zeros((0,row.len()+2));
        }
        //check line with = array - 2
        //println!("dimx {}, dimy {}, row.len{}", dimx, dimy, row.len());
        if dimx != 0 && row.len() != dimx - 2
        {
            //return wrong length error
            return Err(DiffDimERR{wanted: dimx-2, got:input.len()}.into()); 
        }

        ////mirror+1 1st and last elements (bounds conditions)
        row.insert(0, 9);
        row.push(9);
         
        if dimy == 0
        {
            //insert parsed line with elm +1
            let row0: Vec<u32> = row.iter().map(|&x| 9).collect();
            self.world.push_row(ArrayView::from(&row0))?;
        }
        //add vector to world array
        self.world.push_row(ArrayView::from(&row))?;
        
        return Ok(());
    }

    pub fn finish(&mut self)
    {
        //duplicate+1 first and last row
        let mut lastrow: Vec<u32> = self.world.slice(s![-1 as i32,..]).to_vec();
        lastrow.iter_mut().for_each(|x| *x=9);
        self.world.push_row(ArrayView::from(&lastrow));
        //println!("{:?}", self.world);
        //self.world.push_row(ArrayView::from(lastrow));
    }

    pub fn print(&self)
    {
        println!("{:?}", self.world);
    }

    pub fn find_low_points(&self) -> Vec<((usize, usize), u32)>
    {
        //get world dimensions
        let (dimx, dimy) = self.world.dim();
        //iterate points and test for low point
        (1..dimx-1).cartesian_product(1..dimy-1)
                   .par_bridge()
                   .filter(|&(x, y)| self.is_low_point((x,y)))
                   .map(|(x,y)| ((x,y),self.world[(x,y)]))
                   .collect()
    }

    pub fn get_height(&self, coord: (usize, usize)) -> u32
    {
        return self.world[coord];
    }

    fn is_low_point(&self, coord: (usize, usize)) -> bool
    {
        let (x,y) = coord;
        //println!("checking ({},{}): {}", x,y, self.world[coord]);
        self.world[coord] < self.world[(x-1,y)]
        && self.world[coord] < self.world[(x+1,y)]
        && self.world[coord] < self.world[(x,y-1)]
        && self.world[coord] < self.world[(x,y+1)]
    }
}

impl Basin
{
    pub fn calculate(map: Rc<Heatmap>, coord: (usize, usize)) -> Option<Basin>
    {
        //let (x,y) = coord;
        
        let mut candidatelist = vec![coord];
        let mut basin = Basin{lowpoint:coord, points: Vec::new(), map: map.clone()};
       
        while candidatelist.len() > 0
        {
            //println!("{:?}", candidatelist);
            basin.extend(&mut candidatelist);
        }

        if basin.points.len() == 0
        {
            println!("???:{:?} -> {:?}", coord, map.get_height(coord));
            return None;
        }

        println!("basin arround {:?} contains: {:?}", coord, basin.points);
        return Some(basin);
    }

    fn extend(&mut self, candidatelist: &mut Vec<(usize, usize)>)
    {
        //get point to test
        let testpoint = match candidatelist.pop() 
            {
                Some(val) => val,
                None => return,
            };
        
        //if test point already contained.. ignore
        if self.points.contains(&testpoint)
        {
            return;
        }
        
        if self.map.get_height(testpoint) < 9
        {
            self.points.push(testpoint);
            
            let mut newcandidates = vec!(
                                            (testpoint.0 + 1, testpoint.1+0),
                                            (testpoint.0 - 1, testpoint.1+0),
                                            (testpoint.0 + 0, testpoint.1+1),
                                            (testpoint.0 + 0, testpoint.1-1),
                                        );
            newcandidates.retain(|&(x,y)|    x >0 
                                          && y >0 
                                          && ! self.points.contains(&(x,y))
                                          && ! candidatelist.contains(&(x,y)));

            candidatelist.append(&mut newcandidates);
        }
    }

    pub fn size(&self) -> usize
    {
        return self.points.len();
    }
}
