use regex::Regex;
use lazy_static::lazy_static;
use ndarray::*;
use std::cmp::*;
use itertools::Itertools;

pub struct Sheet
{
    pub board: Array2<u8>,
}

impl Sheet
{
    pub fn parse(input: &str) -> Option<Sheet>
    {
        let mut b:Array2::<u8> = Array2::<u8>::zeros((0, 5));
        lazy_static!{
            static ref RE_BOARD: Regex =  Regex::new(r"(?m)^\s*(?P<n1>\d+)\s+(?P<n2>\d+)\s+(?P<n3>\d+)\s+(?P<n4>\d+)\s+(?P<n5>\d+)\s*$").unwrap();
        }

        for caps in RE_BOARD.captures_iter(input)
        {
            b.push_row(ArrayView::from(&[
                                            caps["n1"].parse::<u8>().unwrap(),
                                            caps["n2"].parse::<u8>().unwrap(),
                                            caps["n3"].parse::<u8>().unwrap(),
                                            caps["n4"].parse::<u8>().unwrap(),
                                            caps["n5"].parse::<u8>().unwrap(),
                                        ])).unwrap();
        }
        if b.shape() == &[5, 5]
        {
            return Some(Sheet{board:b,});
        } else
        {
            return None;
        }
    }

    // return (winning round, final/wining score)
    pub fn getScore(&self, draws: &[u8]) -> (Option<u64>, u64)
    {
        #[cfg(test)]
        println!("Board: {}",self.board);
        let mut score: u64 = 0;
        let mut winninground = None;
        let hits = self.board.map(|b| tou64Option(draws.iter().position(|&d| d == *b)));
        #[cfg(test)]
        println!("\tHits: {:?}", hits);
        
        for line in hits.rows()
        {
            winninground = cmpOptions(winninground, line.fold(None, |acc, elm| cmpOptions(acc, *elm, |a,b| Some(max(a,b)))), |a,b| Some(min(a,b)));
        }
        for line in hits.columns()
        {
            winninground = cmpOptions(winninground, line.fold(None, |acc, elm| cmpOptions(acc, *elm, |a,b| Some(max(a,b)))), |a,b| Some(min(a,b)));
        }
        #[cfg(test)]
        println!("\twinninground: {:?}", winninground);

        //calc score
        for (x,y) in (0..5).cartesian_product(0..5)
        {
            if hits.get((x,y)).unwrap().unwrap_or(<u64>::MAX) > winninground.unwrap_or(draws.len() as u64)
            {
                score += self.board.get((x,y)).unwrap().clone() as u64;
            }
        }

        #[cfg(test)]
        {
        println!("\t unmarked sum = {:?}", score);
        println!("\t winning draw = {:?}", draws[winninground.unwrap_or((draws.len() -1) as u64) as usize] as u64);
        }
        score = score * (draws[winninground.unwrap_or((draws.len() -1) as u64) as usize] as u64);
        #[cfg(test)]
        println!("\t final score = {:?}", score);

        (winninground, score)
    }
}

fn tou64Option (op : Option<usize>) -> Option<u64>
{
    if op.is_some()
    {
        Some((op.unwrap()) as u64)
    } else
    {
        None
    }
}
fn cmpOptions<F: FnOnce(u64, u64) -> Option<u64>>(a : Option<u64>, b:Option<u64>, f: F) -> Option<u64>
{
    if a.is_none()
    {
        b
    } else if b.is_none()
    {
        a
    } else
    {
        f(a.unwrap(),b.unwrap())
    }
}
