// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::process::exit;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot{
    x: i32,
    y: i32,
    d: Direction,
}

impl From<usize> for Direction{
    fn from(d: usize) -> Direction {
        match {d%4} {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => unreachable!()
        }
    }
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self{ x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self{ d: Direction::from(self.d as usize +1), ..self}
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self{ d: Direction::from(self.d as usize +3), ..self}
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.d {
            Direction::North => Self{y: self.y+1, ..self },
            Direction::East => Self{x: self.x+1, ..self },
            Direction::South => Self{y: self.y-1, ..self },
            Direction::West => Self{x: self.x-1, ..self },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut r: Robot = self;
        for inst in instructions.chars(){
            match inst{
                'A' => r = r.advance(),
                'L' => r = r.turn_left(),
                'R' => r = r.turn_right(),
                _ => {
                    println!("Istruzione non valida");
                    exit(1);
                }
            };
        }
        r
    }

    pub fn position(&self) -> (i32, i32) {
        return (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        return &self.d
    }
}
