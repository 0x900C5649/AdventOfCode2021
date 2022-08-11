use std::fmt;

#[derive(Debug)]
pub enum NavParseErr{
    IncompleteErr(Box<String>),
    CorruptedErr((char, usize)),
}
impl fmt::Display for NavParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NavParseErr::CorruptedErr((c, idx)) =>
                write!(f, "Corrupted Code at index: {} with Character \'{}\'", idx, c),
            NavParseErr::IncompleteErr(remain) =>
                write!(f, "Incompleted Code: \"{}\"",remain),
        }
    }
}
impl std::error::Error for NavParseErr {}

pub fn check_line(input: &str) -> Result<(), NavParseErr>
{
    let mut parse_stack: Vec<char> = Vec::new();

    for (idx, c) in input.chars().enumerate()
    {
        match c
        {
            '(' | '[' | '{' | '<' => parse_stack.push(c),
            ')' => {
                    match parse_stack.pop()
                    {
                        Some('(') => {},
                        _ => return Err(NavParseErr::CorruptedErr((c, idx))),
                    }
                },
            ']' =>  {
                    match parse_stack.pop()
                    {
                        Some('[') => {},
                        _ => return Err(NavParseErr::CorruptedErr((c, idx))),
                    }
                },
            '}' => {
                    match parse_stack.pop()
                    {
                        Some('{') => {},
                        _ => return Err(NavParseErr::CorruptedErr((c, idx))),
                    }
                },
            '>' => {
                    match parse_stack.pop()
                    {
                        Some('<') => {},
                        _ => return Err(NavParseErr::CorruptedErr((c, idx))),
                    }
                },
            _ => {},
        }
    }
    
    if parse_stack.len() != 0
    {
        return Err(NavParseErr::IncompleteErr(Box::new(String::from_iter(parse_stack))))
    }

    return Ok(());
}

pub fn get_correcting_string(remainder: String) -> String
{
    let mut correcting = Vec::new();

    for c in remainder.chars().rev()
    {
        let m = match c
            {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => ' ',
            };
        correcting.push(m);
    }
    return String::from_iter(correcting);
}

