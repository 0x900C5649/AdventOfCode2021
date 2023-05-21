//std
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct UnsimulatedError
{
    pub wanted: usize,
    pub simulated: usize
}
impl fmt::Display for UnsimulatedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EnergyMap is simulated up to step {}, requested was step {}", self.simulated, self.wanted)
    }
}
impl error::Error for UnsimulatedError {}

#[derive(Debug, Clone)]
pub struct DiffDimERR
{
    pub wanted: usize,
    pub got: usize,
}

impl fmt::Display for DiffDimERR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Incompatible Dimensions: got {}, expected {}", self.got, self.wanted)
    }
}
impl error::Error for DiffDimERR {}

#[derive(Debug, Clone)]
pub struct AlreadyInitedERR{}
impl fmt::Display for AlreadyInitedERR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Data Structure is already initialized")
    }
}
impl error::Error for AlreadyInitedERR {}
