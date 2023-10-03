mod dirasenum;
mod summary_example;

use std::convert::From;
use std::fs;
use std::ops::Add;

#[derive(Debug, Default, Clone, Copy)]
pub struct Direction {
    x: i32,
    y: i32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<Direction> for Coordinate {
    fn from(value: Direction) -> Self {
        Coordinate {
            x: value.x,
            y: value.y,
        }
    }
}

impl Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Direction) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Default)]
pub struct Robot {
    position: Coordinate,
}

impl Robot {
    fn move_in_direction(&mut self, direction: Direction) {
        self.position = self.position + direction;
    }
}

#[derive(Debug)]
pub enum Command {
    Move(Direction),
}

impl Command {
    pub fn from_str(command_str: &str) -> Option<Self> {
        match command_str {
            "UP" => Some(Command::Move(Direction { x: 0, y: 1 })),
            "DOWN" => Some(Command::Move(Direction { x: 0, y: -1 })),
            "LEFT" => Some(Command::Move(Direction { x: -1, y: 0 })),
            "RIGHT" => Some(Command::Move(Direction { x: 1, y: 0 })),
            _ => None,
        }
    }
}

fn main() {
    let content = fs::read_to_string("commands.txt").expect("Unable to read file");

    let commands: Vec<Command> = content.lines().filter_map(Command::from_str).collect();

    let mut robot = Robot::default();
    for command in commands {
        match command {
            Command::Move(direction) => robot.move_in_direction(direction),
        }
    }

    println!("Robot's final position: {:?}", robot.position);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
