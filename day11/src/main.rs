use intcode::{Program, Value};
use std::collections::HashSet;
use std::fmt::{Display, Error, Formatter};

const HULL_SIZE: usize = 100;

fn main() {
    let input: Vec<Value> = vec![
        3,
        8,
        1005,
        8,
        338,
        1106,
        0,
        11,
        0,
        0,
        0,
        104,
        1,
        104,
        0,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        1,
        10,
        4,
        10,
        1002,
        8,
        1,
        29,
        2,
        105,
        19,
        10,
        1006,
        0,
        52,
        1,
        1009,
        7,
        10,
        1006,
        0,
        6,
        3,
        8,
        102,
        -1,
        8,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        1001,
        8,
        0,
        64,
        2,
        1002,
        19,
        10,
        1,
        8,
        13,
        10,
        1,
        1108,
        16,
        10,
        2,
        1003,
        1,
        10,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        1,
        10,
        4,
        10,
        1002,
        8,
        1,
        103,
        1006,
        0,
        10,
        2,
        109,
        16,
        10,
        1,
        102,
        11,
        10,
        2,
        6,
        13,
        10,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        0,
        10,
        4,
        10,
        1002,
        8,
        1,
        140,
        2,
        102,
        8,
        10,
        2,
        4,
        14,
        10,
        1,
        8,
        19,
        10,
        1006,
        0,
        24,
        3,
        8,
        1002,
        8,
        -1,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        1008,
        8,
        0,
        10,
        4,
        10,
        1001,
        8,
        0,
        177,
        1006,
        0,
        16,
        1,
        1007,
        17,
        10,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        101,
        0,
        8,
        205,
        3,
        8,
        1002,
        8,
        -1,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        0,
        10,
        4,
        10,
        102,
        1,
        8,
        228,
        1,
        1005,
        1,
        10,
        1,
        9,
        1,
        10,
        3,
        8,
        102,
        -1,
        8,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        1008,
        8,
        1,
        10,
        4,
        10,
        1002,
        8,
        1,
        258,
        3,
        8,
        1002,
        8,
        -1,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        0,
        8,
        10,
        4,
        10,
        102,
        1,
        8,
        279,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        0,
        8,
        10,
        4,
        10,
        102,
        1,
        8,
        301,
        1,
        3,
        17,
        10,
        2,
        7,
        14,
        10,
        2,
        6,
        18,
        10,
        1,
        1001,
        17,
        10,
        101,
        1,
        9,
        9,
        1007,
        9,
        1088,
        10,
        1005,
        10,
        15,
        99,
        109,
        660,
        104,
        0,
        104,
        1,
        21102,
        1,
        48_092_525_312,
        1,
        21101,
        355,
        0,
        0,
        1106,
        0,
        459,
        21102,
        665_750_184_716,
        1,
        1,
        21102,
        366,
        1,
        0,
        1106,
        0,
        459,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        1,
        21102,
        1,
        235_324_768_296,
        1,
        21101,
        0,
        413,
        0,
        1105,
        1,
        459,
        21101,
        3_263_212_736,
        0,
        1,
        21102,
        424,
        1,
        0,
        1106,
        0,
        459,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        0,
        21102,
        1,
        709_496_824_676,
        1,
        21101,
        447,
        0,
        0,
        1105,
        1,
        459,
        21102,
        988_220_904_204,
        1,
        1,
        21102,
        1,
        458,
        0,
        1106,
        0,
        459,
        99,
        109,
        2,
        21201,
        -1,
        0,
        1,
        21102,
        40,
        1,
        2,
        21102,
        490,
        1,
        3,
        21102,
        1,
        480,
        0,
        1105,
        1,
        523,
        109,
        -2,
        2106,
        0,
        0,
        0,
        1,
        0,
        0,
        1,
        109,
        2,
        3,
        10,
        204,
        -1,
        1001,
        485,
        486,
        501,
        4,
        0,
        1001,
        485,
        1,
        485,
        108,
        4,
        485,
        10,
        1006,
        10,
        517,
        1101,
        0,
        0,
        485,
        109,
        -2,
        2105,
        1,
        0,
        0,
        109,
        4,
        2101,
        0,
        -1,
        522,
        1207,
        -3,
        0,
        10,
        1006,
        10,
        540,
        21102,
        0,
        1,
        -3,
        22101,
        0,
        -3,
        1,
        22102,
        1,
        -2,
        2,
        21102,
        1,
        1,
        3,
        21101,
        559,
        0,
        0,
        1106,
        0,
        564,
        109,
        -4,
        2105,
        1,
        0,
        109,
        5,
        1207,
        -3,
        1,
        10,
        1006,
        10,
        587,
        2207,
        -4,
        -2,
        10,
        1006,
        10,
        587,
        22102,
        1,
        -4,
        -4,
        1105,
        1,
        655,
        22101,
        0,
        -4,
        1,
        21201,
        -3,
        -1,
        2,
        21202,
        -2,
        2,
        3,
        21102,
        606,
        1,
        0,
        1105,
        1,
        564,
        21202,
        1,
        1,
        -4,
        21101,
        0,
        1,
        -1,
        2207,
        -4,
        -2,
        10,
        1006,
        10,
        625,
        21102,
        0,
        1,
        -1,
        22202,
        -2,
        -1,
        -2,
        2107,
        0,
        -3,
        10,
        1006,
        10,
        647,
        22101,
        0,
        -1,
        1,
        21101,
        647,
        0,
        0,
        105,
        1,
        522,
        21202,
        -2,
        -1,
        -2,
        22201,
        -4,
        -2,
        -4,
        109,
        -5,
        2106,
        0,
        0,
    ];
    let mut hull = Hull::new(&input);
    while let Some(_) = hull.next() {
        //        println!("Position: {} {}", hull.robot_x, hull.robot_y);
    }
    println!("Visited: {}", hull.painted.len());

    let mut hull = Hull::new_with_starting_tile(&input, Tile::White);
    while let Some(_) = hull.next() {
        //        println!("Position: {} {}", hull.robot_x, hull.robot_y);
    }
    println!("Hull:\n{}", hull);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Black,
    White,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Orientation {
    North,
    West,
    East,
    South,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl Orientation {
    fn turn(self, direction: Direction) -> Self {
        match direction {
            Direction::Left => match self {
                Orientation::North => Orientation::West,
                Orientation::West => Orientation::South,
                Orientation::South => Orientation::East,
                Orientation::East => Orientation::North,
            },
            Direction::Right => match self {
                Orientation::North => Orientation::East,
                Orientation::East => Orientation::South,
                Orientation::South => Orientation::West,
                Orientation::West => Orientation::North,
            },
        }
    }
}

struct Hull {
    tiles: Vec<Vec<Tile>>,
    robot: Robot,
    robot_x: usize,
    robot_y: usize,
    painted: HashSet<(usize, usize)>,
}

impl Hull {
    fn new(data: &[Value]) -> Self {
        let tiles = vec![vec![Tile::Black; HULL_SIZE]; HULL_SIZE];
        let initial_pos = HULL_SIZE / 2;
        Hull {
            tiles,
            robot: Robot::new(data),
            robot_x: initial_pos,
            robot_y: initial_pos,
            painted: HashSet::new(),
        }
    }
    fn new_with_starting_tile(data: &[Value], tile: Tile) -> Self {
        let mut hull = Hull::new(data);
        hull.set_tile(tile);
        hull
    }
    fn get_tile(&self) -> Option<Tile> {
        self.tiles.get(self.robot_y)?.get(self.robot_x).cloned()
    }
    fn set_tile(&mut self, color: Tile) {
        let column = self.tiles.get_mut(self.robot_y).unwrap();
        let tile = column.get_mut(self.robot_x).unwrap();
        *tile = color;
    }
    fn paint_tile(&mut self, color: Tile) {
        self.set_tile(color);
        self.painted.insert((self.robot_x, self.robot_y));
    }
    fn turn_robot(&mut self, direction: Direction) {
        self.robot.turn(direction);
    }
    fn move_robot_forward(&mut self) {
        match self.robot.orientation {
            Orientation::North => self.robot_y -= 1,
            Orientation::West => self.robot_x -= 1,
            Orientation::East => self.robot_x += 1,
            Orientation::South => self.robot_y += 1,
        }
    }
    fn next(&mut self) -> Option<()> {
        let input = match self.get_tile().unwrap() {
            Tile::Black => 0,
            Tile::White => 1,
        };
        // This returns None if the program has halted
        let (color, direction) = self.robot.think(input)?;
        self.paint_tile(color);
        self.turn_robot(direction);
        self.move_robot_forward();
        // This returns Some(()) if there is more to come
        Some(())
    }
}

impl Display for Hull {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for row in &self.tiles {
            for tile in row {
                match tile {
                    Tile::Black => write!(f, " ")?,
                    Tile::White => write!(f, "█")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Robot {
    brain: Program,
    orientation: Orientation,
}

impl Robot {
    fn new(data: &[Value]) -> Self {
        let brain = Program::new(&data);
        Robot {
            brain,
            orientation: Orientation::North,
        }
    }
    fn turn(&mut self, direction: Direction) {
        self.orientation = self.orientation.turn(direction);
    }
    /// this function returns None if the programm has halted, Tile and Direction otherwise
    fn think(&mut self, input: Value) -> Option<(Tile, Direction)> {
        self.brain.set_input(input);
        // first output is the tile color 0 for black, 1 for white
        let color = match self.brain.run_pipe()? {
            0 => Tile::Black,
            1 => Tile::White,
            _ => panic!("Unknown tile color!"),
        };
        // second output is the direction to turn, 0 means left, 1 means right
        let direction = match self.brain.run_pipe()? {
            0 => Direction::Left,
            1 => Direction::Right,
            _ => panic!("Unknown direction!"),
        };
        Some((color, direction))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
