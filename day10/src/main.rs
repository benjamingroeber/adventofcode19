use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::f64::consts::PI;

fn main() {
    let input = "#...##.####.#.......#.##..##.#.
#.##.#..#..#...##..##.##.#.....
#..#####.#......#..#....#.###.#
...#.#.#...#..#.....#..#..#.#..
.#.....##..#...#..#.#...##.....
##.....#..........##..#......##
.##..##.#.#....##..##.......#..
#.##.##....###..#...##...##....
##.#.#............##..#...##..#
###..##.###.....#.##...####....
...##..#...##...##..#.#..#...#.
..#.#.##.#.#.#####.#....####.#.
#......###.##....#...#...#...##
.....#...#.#.#.#....#...#......
#..#.#.#..#....#..#...#..#..##.
#.....#..##.....#...###..#..#.#
.....####.#..#...##..#..#..#..#
..#.....#.#........#.#.##..####
.#.....##..#.##.....#...###....
###.###....#..#..#.....#####...
#..##.##..##.#.#....#.#......#.
.#....#.##..#.#.#.......##.....
##.##...#...#....###.#....#....
.....#.######.#.#..#..#.#.....#
.#..#.##.#....#.##..#.#...##..#
.##.###..#..#..#.###...#####.#.
#...#...........#.....#.......#
#....##.#.#..##...#..####...#..
#.####......#####.....#.##..#..
.#...#....#...##..##.#.#......#
#..###.....##.#.......#.##...##
";
    let map = Map::new(&input);
    let best = map.get_best_observer().unwrap();
    println!("Max visibility: {:?} with {:?}", best.0, best.1);

    let vaporization_order = map.get_vaporization_order(best.0);
    let twohundredth = vaporization_order[199];
    println!("Point: {:?}, Solution: {}",  twohundredth, twohundredth.position.x*100+twohundredth.position.y);
}

struct Map {
    content: Vec<Vec<Tile>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    /// This function returns the angle of the `other` Position to `self` starting from
    /// the positive y axis.
    fn angle_to(&self, other: &Position) -> f64 {
        let absolute_x = self.x as f64 - other.x as f64;
        let absolute_y = self.y as f64 - other.y as f64;
        // Change orientation, such that angle is relative to positive y axis
        // Pi/2 needs to be SUBTRACTED to rotate the starting point 0 counterclockwise
        // onto the y axis
        let mut angle = absolute_y.atan2(absolute_x) - (PI/2.0);
        // Adding a whole rotation to negative angles, such that 0 is the starting point
        if angle < 0.0 {
            angle += std::f64::consts::PI * 2.0;
        }
        angle
    }
    // This function returns the distance of the `other` Position to `self`.
    fn distance_from(&self, other: &Position) -> f64 {
        let absolute_x = self.x as f64 - other.x as f64;
        let absolute_y = self.y as f64 - other.y as f64;
        absolute_y.hypot(absolute_x)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct RelativePosition {
    angle: f64,
    distance: f64,
    position: Position,
}

impl Map {
    fn new(input: &str) -> Self {
        let content: Vec<Vec<Tile>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .filter(|c| *c == '#' || *c == '.')
                    .map(|c| c.into())
                    .collect()
            })
            .collect();
        Map { content }
    }
    fn get_asteroids(&self) -> Vec<Position> {
        let mut asteroids = Vec::new();
        for (y, row) in self.content.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let Tile::Asteroid = tile {
                    asteroids.push(Position { x, y });
                }
            }
        }
        asteroids
    }
    fn get_visibility_count_all(&self) -> HashMap<Position, usize> {
        let asteroids = self.get_asteroids();
        let mut visibility = HashMap::new();
        for asteroid in &asteroids {
            // get the relative angle for all asteroids
            let mut relative_positions = self.get_relative_positions(*asteroid);

            // we can see only one object for each unique angle
            relative_positions.sort_by(|a, b| a.angle.partial_cmp(&b.angle).unwrap());
            relative_positions.dedup_by_key(|rd| rd.angle);

            visibility.insert(*asteroid, relative_positions.len());
        }
        visibility
    }
    fn get_relative_positions(&self, asteroid: Position) -> Vec<RelativePosition> {
        let asteroids = self.get_asteroids();
        // get the relative angle for all asteroids
        asteroids
            .iter()
            // don't return us
            .filter(|other| asteroid != **other)
            .map(|other| RelativePosition {
                angle: asteroid.angle_to(&other),
                distance: asteroid.distance_from(&other),
                position: *other,
            })
            .collect()
    }
    fn get_best_observer(&self) -> Option<(Position, usize)> {
        let visibilities = self.get_visibility_count_all();
        visibilities
            .iter()
            .max_by_key(|(_, count)| **count)
            .map(|(p, c)| (*p, *c))
    }
    /// Vaporizing starts always UP and clockwise
    /// a beam destroys only one asteroid per angle.
    /// This function returns the ordered list of vaporized Asteroids
    fn get_vaporization_order(&self, position: Position) -> Vec<RelativePosition> {
        let mut other_asteroids = self.get_relative_positions(position);
        // sort asteroids by distance
        other_asteroids.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

        // get the order for each angle
        let mut angles_order: HashMap<isize, VecDeque<RelativePosition>> = HashMap::new();
        for relative_position in &other_asteroids {
            let angle = (relative_position.angle * 1_000.0).round() as isize;
            let angle_order = angles_order.entry(angle).or_insert_with(VecDeque::new);
            angle_order.push_back(*relative_position)
        }

        let mut global_positions = Vec::new();
        let mut keys:Vec<_> = angles_order.keys().cloned().collect();
        keys.sort();

        loop {
            let mut empty = true;
            for angle in &keys{
                let positions = angles_order.get_mut(angle).unwrap();

                if !positions.is_empty() {
                    empty = false;
                    global_positions.push(positions.pop_front().unwrap())
                }
            }
            if empty {
                break
            }
        }
        global_positions
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Asteroid,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Asteroid,
            '.' => Tile::Empty,
            _ => panic!("Unrecognized Tile"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vaporization_order_example() {
        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        let map = Map::new(&input);

        let vaporization_order = map.get_vaporization_order(Position { x: 11, y: 13 });
        assert_eq!(vaporization_order[0].position, Position { x: 11, y: 12 });
        assert_eq!(vaporization_order[1].position, Position { x: 12, y: 1 });
        assert_eq!(vaporization_order[2].position, Position { x: 12, y: 2 });

        assert_eq!(vaporization_order[9].position, Position { x: 12, y: 8 });
        assert_eq!(vaporization_order[19].position, Position { x: 16, y: 0 });
        assert_eq!(vaporization_order[49].position, Position { x: 16, y: 9 });

        assert_eq!(vaporization_order[99].position, Position { x: 10, y: 16 });
        assert_eq!(vaporization_order[198].position, Position { x: 9, y: 6 });
        assert_eq!(vaporization_order[199].position, Position { x: 8, y: 2 });
        assert_eq!(vaporization_order[200].position, Position { x: 10, y: 9 });

        assert_eq!(vaporization_order[298].position, Position { x: 11, y: 1 });
    }

    #[test]
    fn test_example_text() {
        let input = ".#..#
.....
#####
....#
...##";
        let map = Map::new(&input);
        let best = map.get_best_observer().unwrap();

        assert_eq!(best.1, 8);
        assert_eq!(best.0.x, 3);
        assert_eq!(best.0.y, 4);
    }

    #[test]
    fn test_example_1() {
        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        let map = Map::new(&input);
        let best = map.get_best_observer().unwrap();

        assert_eq!(best.0.x, 5);
        assert_eq!(best.0.y, 8);
        assert_eq!(best.1, 33);
    }

    #[test]
    fn test_example_2() {
        let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        let map = Map::new(&input);
        let best = map.get_best_observer().unwrap();

        assert_eq!(best.0.x, 1);
        assert_eq!(best.0.y, 2);
        assert_eq!(best.1, 35);
    }

    #[test]
    fn test_example_3() {
        let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
        let map = Map::new(&input);
        let best = map.get_best_observer().unwrap();

        assert_eq!(best.0.x, 6);
        assert_eq!(best.0.y, 3);
        assert_eq!(best.1, 41);
    }

    #[test]
    fn test_example_4() {
        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        let map = Map::new(&input);
        let best = map.get_best_observer().unwrap();

        assert_eq!(best.0.x, 11);
        assert_eq!(best.0.y, 13);
        assert_eq!(best.1, 210);
    }

    #[test]
    fn test_get_asteroids() {
            let input = "##
.#";
            let map = Map::new(input);
            let asteroids = map.get_asteroids();

            let expected = vec![
                Position { x: 0, y: 0 },
                Position { x: 1, y: 0 },
                Position { x: 1, y: 1 },
            ];

            assert_eq!(expected, asteroids);

    }

    #[test]
    fn test_parse_tile() {
        let asteroid: Tile = '#'.into();
        let empty: Tile = '.'.into();

        assert_eq!(asteroid, Tile::Asteroid);
        assert_eq!(empty, Tile::Empty);
    }

    #[test]
    fn test_get() {
        let input = "##
..";
        let map = Map::new(input);

        assert_eq!(map.content[0][0], Tile::Asteroid);
        assert_eq!(map.content[1][1], Tile::Empty);
    }

    #[test]
    fn test_example1() {
        let input = ".#..#
.....
#####
....#
...##";
        let map = Map::new(input);
    }
}
