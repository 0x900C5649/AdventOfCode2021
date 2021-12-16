use regex::Regex; 
use lazy_static::lazy_static;

pub enum Command
{
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl Command
{
    pub fn parse(input: &str) -> Option<Command>
    {
        lazy_static! {
            static ref RE_Commands:Regex = Regex::new(r"^(?P<cmd>\w+) (?P<amount>\d+)$").unwrap();
        }
        if let Some(captures) = RE_Commands.captures(input)
        {
            let amount:i64 = captures.name("amount").unwrap().as_str().parse::<i64>().unwrap();
            match captures.name("cmd").unwrap().as_str()
            {
                "forward"=> return Some(Command::Forward(amount)),
                "up" => return Some(Command::Up(amount)),
                "down" =>return Some(Command::Down(amount)),
                _ => return None,
            };
        }
        None
    }
}

pub struct Submarine
{
    pub position:i64,
    pub depth:i64,
    pub aim:i64,
}

impl Submarine
{
    pub fn new() -> Submarine
    {
        Submarine{
            position: 0,
            depth: 0,
            aim:0,
            }
    }

    pub fn execute_p1(&mut self, cmd: Command)
    {
        match cmd
        {
            Command::Forward(x) => self.position += x,
            Command::Up(x) => self.depth -= x,
            Command::Down(x) => self.depth += x,
        }
    }

    pub fn execute_p2(&mut self, cmd: Command)
    {
        match cmd
        {
            Command::Forward(x) =>
            {
                self.position += x;
                self.depth += self.aim * x;
            },
            Command::Up(x) =>
            {
                self.aim -= x;
            }
            Command::Down(x) =>
            {
                self.aim += x;
            },
        }
    }
}
