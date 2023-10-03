use std::convert::From;
use std::fs;
use std::ops::Add;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "UP" => Ok(Direction::Up),
            "DOWN" => Ok(Direction::Down),
            "LEFT" => Ok(Direction::Left),
            "RIGHT" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl From<Direction> for Coordinate {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Coordinate { x: 0, y: 1 },
            Direction::Down => Coordinate { x: 0, y: -1 },
            Direction::Left => Coordinate { x: 1, y: 0 },
            Direction::Right => Coordinate { x: -1, y: 0 },
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Into::<Coordinate>::into(rhs)
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

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Move(Direction),
}

impl Command {
    pub fn from_str(command_str: &str) -> Option<Self> {
        if let Ok(dir) = Direction::try_from(command_str) {
            Some(Command::Move(dir))
        } else {
            None
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
    fn test_command_parsing() {
        assert_eq!(Command::from_str("UP"), Some(Command::Move(Direction::Up)));
        assert_eq!(
            Command::from_str("DOWN"),
            Some(Command::Move(Direction::Down))
        );
        assert_eq!(
            Command::from_str("LEFT"),
            Some(Command::Move(Direction::Left))
        );
        assert_eq!(
            Command::from_str("RIGHT"),
            Some(Command::Move(Direction::Right))
        );
        assert_eq!(Command::from_str("INVALID"), None);
    }

    #[test]
    fn test_direction_to_coordinate() {
        let up_coord: Coordinate = Direction::Up.into();
        let down_coord: Coordinate = Direction::Down.into();
        let left_coord: Coordinate = Direction::Left.into();
        let right_coord: Coordinate = Direction::Right.into();

        assert_eq!(up_coord, Coordinate { x: 0, y: 1 });
        assert_eq!(down_coord, Coordinate { x: 0, y: -1 });
        assert_eq!(left_coord, Coordinate { x: 1, y: 0 });
        assert_eq!(right_coord, Coordinate { x: -1, y: 0 });
    }

    #[test]
    fn test_robot_movement() {
        let mut robot = Robot::default();

        robot.move_in_direction(Direction::Up);
        assert_eq!(robot.position, Coordinate { x: 0, y: 1 });

        robot.move_in_direction(Direction::Down);
        assert_eq!(robot.position, Coordinate { x: 0, y: 0 });

        robot.move_in_direction(Direction::Left);
        assert_eq!(robot.position, Coordinate { x: 1, y: 0 });

        robot.move_in_direction(Direction::Right);
        assert_eq!(robot.position, Coordinate { x: 0, y: 0 });
    }

    // If needed, add more tests or edge cases you can think of.
}
