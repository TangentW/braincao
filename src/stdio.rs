use crate::IO;
use std::io;
use std::io::{Read, Write};

pub struct Stdio;

impl Default for Stdio {
    fn default() -> Self {
        Self
    }
}

impl IO for Stdio {
    type Err = io::Error;

    fn input(&mut self) -> Result<i32, Self::Err> {
        let mut input: [u8; 1] = [0];
        io::stdin().read(&mut input)?;
        Ok(input[0] as i32)
    }

    fn output(&mut self, val: i32) -> Result<(), Self::Err> {
        print!("{}", val as u8 as char);
        io::stdout().flush()
    }
}
