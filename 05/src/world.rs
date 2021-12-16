use hashbrown::HashMap;
use euclid::*;
use regex::Regex;
use lazy_static::lazy_static;

pub struct Line
{
    pub p1: Point2D<f32, UnknownUnit>,
    pub p2: Point2D<f32, UnknownUnit>,
}


impl Line
{
    pub fn parse(input: &str) ->Option<Line>
    {
        lazy_static!{
            static ref RE_LINE: Regex = Regex::new(r"^\s*(?P<x1>\d+),(?P<y1>\d+)\s*->\s*(?P<x2>\d+),(?P<y2>\d+)$").unwrap();
        }

        if let Some(caps) = RE_LINE.captures(input)
        {
            let l1 = point2(
                            caps["x1"].parse::<u64>().unwrap() as f32,
                            caps["y1"].parse::<u64>().unwrap() as f32
                            );

            let l2 = point2(
                            caps["x2"].parse::<u64>().unwrap() as f32,
                            caps["y2"].parse::<u64>().unwrap() as f32
                            );
            return Some(Line{p1:l1, p2:l2});
        }
        None
    }

    pub fn dir(&self) -> Vector2D<f32, UnknownUnit>
    {
        let mut vec = self.p2-self.p1;
        vec.x = if vec.x > 0 as f32 {1 as f32} else {if vec.x == 0 as f32 {0 as f32} else{-1 as f32}};
        vec.y = if vec.y > 0 as f32 {1 as f32} else {if vec.y == 0 as f32 {0 as f32} else{-1 as f32}};
        return vec;
    }

    pub fn dist(&self) -> f32
    {
        let vec = self.p2-self.p1;
        return vec.length();
    }
    
    pub fn is_orthogonal(&self) ->bool
    {
        let norm = self.dir();
        return norm.x == 0 as f32 || norm.y == 0 as f32;
    }
}

impl<'a> IntoIterator for &'a Line {
    type Item = Point2D<f32, UnknownUnit>;
    type IntoIter = PointIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PointIterator {
            line: self,
            index: 0 as f32,
        }
    }
}

pub struct PointIterator<'a>
{
    line: &'a Line,
    index: f32,
}

impl<'a> Iterator for PointIterator<'a> {
    type Item = Point2D<f32, UnknownUnit>;
    fn next(&mut self) -> Option<Point2D<f32, UnknownUnit>> {
        let dir = self.line.dir();
        //if self.index > (self.line.dist())
        //{
            //return None;
        //}
        let result = self.line.p1 + dir*self.index;
        if (dir*self.index).length() > self.line.dist()
        {
            return None;
        }
        self.index+=1 as f32;
        Some(result)
    }
}

pub struct World
{
    pub inner: HashMap<(u32,u32), u32>, 
}

impl World
{
    pub fn new() -> World
    {
        World{inner: HashMap::new(), }
    }

    pub fn add_line(&mut self, l:Line)
    {
        for p in l.into_iter()
        {
            let counter = self.inner.entry((p.x as u32, p.y as u32)).or_insert(0 as u32);
            *counter +=1;
        }
    }

    pub fn get_intersections(&self)->Vec<(u32, u32)>
    {
        let mut intersections = Vec::new();
        for (coord, crossings) in self.inner.iter()
        {
            if *crossings > 1
            {
                intersections.push(coord.clone());
            }
        }
        return intersections;
    }
}

