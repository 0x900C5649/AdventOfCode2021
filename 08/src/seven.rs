use lazy_static::lazy_static;
use regex::Regex;
use std::string::String;
use std::collections::HashMap;
use std::cmp::min;
use itertools::*;

pub fn resolve_wiring(input: &str) -> u64 {
    lazy_static! {
        static ref RE_DIGITS: Regex =
            Regex::new(r"^\s*(\w+)\s+(\w+)\s+(\w+)\s+(\w+)\s+(\w+)\s+(\w+)\s+(\w+)\s+(\w+)\s+(\w+)\s+(\w+)\s*\|\s*(\w+)\s+(\w+)\s+(\w+)\s+(\w+)\s*$").unwrap();
    }
    let mut sp = Vec::new();
    let mut digits = Vec::new();
    //let mut histogram: [u32;8] = [0;8];

    if let Some(captures) = RE_DIGITS.captures(input) {
        for capidx in 1..11
        {
            sp.push(captures[capidx].to_string());
            //histogram[captures[capidx].len()] += 1;
        }
        
        for capidx in 1..5
        {
            digits.push(captures[10+capidx].to_string());
        }
    }
    
    let hints = get_mapping(&sp);

    //println!("{:?}", histogram);
    decode(&digits, &hints)
}

fn get_mapping(signals: &Vec<String>) -> Vec<(char, char)>
{
    //let mut msig = &signals.clone();
    let mut mappings : Vec<(char,char)> = Vec::new();
    let cf:String;
    let bd:String;
    let eg:String;

    //resolve unique lengths
    // 2
    if let Some(s) = signals.iter().find(|x| x.len() == 2)
    {
        assert_eq!(s.len(), 2);
        cf = s.clone();
    }
    else
    {
        panic!("couldn't find length 2");
    }
    
    // 4
    if let Some(s) = signals.iter().find(|x| x.len() == 4)
    {
        let mut k = s.clone();
        for c in cf.chars()
        {
            k = k.replace(c, "");
        }
        assert_eq!(k.len(), 2);
        bd = k;
    }
    else
    {
        panic!("couldn't find length 4");
    }
    
    // 7
    if let Some(s) = signals.iter().find(|x| x.len() == 3)
    {
        let mut k = s.clone();
        for c in cf.chars()
        {
            k = k.replace(c, "");
        }
        assert_eq!(k.len(), 1);
        mappings.push((k.chars().next().unwrap(), 'a'));
    }
    else
    {
        panic!("couldn't find length 3");
    }

    // 8
    if let Some(s) = signals.iter().find(|x| x.len() == 7)
    {
        let mut k = s.clone();
        for c in chain!(cf.chars(), bd.chars())
        {
            k = k.replace(c, "");
        }
        eg = k.replace(mappings[0].0, ""); //replace a
        assert_eq!(eg.len(), 2);
    }
    else
    {
        panic!("couldn't find length 7");
    }

    //resolve length 6
    for s in signals.iter().filter(|&x| x.len() == 6)
    {   
        // 0
        if ! s.contains(bd.chars().nth(0).unwrap())
        {
            mappings.push((bd.chars().nth(0).unwrap(), 'd'));
            mappings.push((bd.chars().nth(1).unwrap(), 'b'));
        } else if ! s.contains(bd.chars().nth(1).unwrap())
        {
            mappings.push((bd.chars().nth(0).unwrap(), 'b'));
            mappings.push((bd.chars().nth(1).unwrap(), 'd'));
        }
        // 6
        else if ! s.contains(cf.chars().nth(0).unwrap())
        {
            mappings.push((cf.chars().nth(0).unwrap(), 'c'));
            mappings.push((cf.chars().nth(1).unwrap(), 'f'));
        }
        else if ! s.contains(cf.chars().nth(1).unwrap())
        {
            mappings.push((cf.chars().nth(0).unwrap(), 'f'));
            mappings.push((cf.chars().nth(1).unwrap(), 'c'));
        }
        else if ! s.contains(eg.chars().nth(0).unwrap())
        {
            mappings.push((eg.chars().nth(0).unwrap(), 'e'));
            mappings.push((eg.chars().nth(1).unwrap(), 'g'));
        }
        else if ! s.contains(eg.chars().nth(1).unwrap())
        {
            mappings.push((eg.chars().nth(0).unwrap(), 'g'));
            mappings.push((eg.chars().nth(1).unwrap(), 'e'));
        }

    }
    
    assert_eq!(mappings.len(), 7);

    print!("mappings are: ");
    for (b,a) in &mappings
    {
        print!("{} -> {}," , b, a);
    }
    println!("");
    
    mappings
}

fn decode(rawdigits: &[String], hints: &[(char, char)]) -> u64
{
    lazy_static! {
        static ref SEVEN2DIGIT: HashMap<u64, u64> = {
            let mut m = HashMap::new();
            m.insert(seven2number(&"abcefg"), 0);
            m.insert(seven2number(&"cf"), 1);
            m.insert(seven2number(&"acdeg"), 2);
            m.insert(seven2number(&"acdfg"), 3);
            m.insert(seven2number(&"bcdf"), 4);
            m.insert(seven2number(&"abdfg"), 5);
            m.insert(seven2number(&"abdefg"), 6);
            m.insert(seven2number(&"acf"), 7);
            m.insert(seven2number(&"abcdefg"), 8);
            m.insert(seven2number(&"abcdfg"), 9);
            m
        };
    }
    let mut res:u64 = 0;
    let mut sevendigits = Vec::<String>::new();

    //replace wrong by write encoding
    for rd in rawdigits
    {
        let mut sevendigit = String::new();
        for (before, after) in hints
        {
            if rd.contains(*before)
            {
                sevendigit.push(*after);
            }
        }
        sevendigits.push(sevendigit);
    }


    for sd in sevendigits
    {
        res *= 10;
        res += SEVEN2DIGIT.get(&seven2number(&sd)).unwrap();
    }
    res
}

fn seven2number(input: &str) -> u64
{
    let mut bytes: [u8;8] = [0;8];
    let strbytes = input.as_bytes();
    for idx in 0..min(8, strbytes.len())
    {
        bytes[idx] = strbytes[idx];
    }
    bytes.sort();
    u64::from_be_bytes(bytes)
}
