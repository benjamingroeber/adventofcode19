fn main() {
    let moons = moons_from_str(
        "<x=3, y=3, z=0>
<x=4, y=-16, z=2>
<x=-10, y=-6, z=5>
<x=-3, y=0, z=-13>",
    );
    let mut system = System::new(moons);
    while system.step() < 1000 {}
    println!("Total Energy: {}", system.total_energy());
}

type Value = i32;
const DIMENSIONS: usize = 3;
#[derive(Debug, Clone, PartialEq)]
struct Moon {
    position: [Value; DIMENSIONS],
    velocity: [Value; DIMENSIONS],
}

impl Moon {
    fn new(position: [Value; DIMENSIONS]) -> Self {
        Moon {
            position,
            velocity: [0; DIMENSIONS],
        }
    }
    /// The total energy for a single moon is its potential energy multiplied by its kinetic energy.
    fn total_energy(&self) -> Value {
        self.potential_energy() * self.kinetic_energy()
    }
    /// A moon's potential energy is the sum of the absolute values of its x, y, and z position coordinates.
    fn potential_energy(&self) -> Value {
        self.position.iter().map(|i| i.abs()).sum()
    }
    /// A moon's kinetic energy is the sum of the absolute values of its velocity coordinates.
    fn kinetic_energy(&self) -> Value {
        self.velocity.iter().map(|i| i.abs()).sum()
    }
}

struct System {
    moons: Vec<Moon>,
    step: usize,
}

impl System {
    fn new(moons: Vec<Moon>) -> Self {
        Self { moons, step: 0 }
    }
    /// Simulate the motion of the moons in time steps. Within each time step, first update the
    /// velocity of every moon by applying gravity. Then, once all moons' velocities have been
    /// updated, update the position of every moon by applying velocity. Time progresses by one
    /// step once all of the positions are updated.
    fn step(&mut self) -> usize {
        // apply gravity to pairs only
        for i in 0..self.moons.len() {
            for j in i + 1..self.moons.len() {
                self.apply_gravity(i, j)
            }
        }
        self.apply_velocity();
        self.step += 1;
        self.step
    }
    fn total_energy(&self) -> Value {
        self.moons.iter().map(|m| m.total_energy()).sum()
    }
    /// To apply gravity, consider every pair of moons. On each axis (x, y, and z), the velocity of
    /// each moon changes by exactly +1 or -1 to pull the moons together. For example, if Ganymede
    /// has an x position of 3, and Callisto has a x position of 5, then Ganymede's x velocity
    /// changes by +1 (because 5 > 3) and Callisto's x velocity changes by -1 (because 3 < 5).
    ///
    /// However, if the positions on a given axis are the same, the velocity on that axis does not
    /// change for that pair of moons.
    fn apply_gravity(&mut self, moon_idx: usize, other_idx: usize) {
        // using indices to avoid borrowing problems
        for i in 0..DIMENSIONS {
            if self.moons[moon_idx].position[i] > self.moons[other_idx].position[i] {
                self.moons[moon_idx].velocity[i] -= 1;
                self.moons[other_idx].velocity[i] += 1;
            } else if self.moons[moon_idx].position[i] < self.moons[other_idx].position[i] {
                self.moons[moon_idx].velocity[i] += 1;
                self.moons[other_idx].velocity[i] -= 1;
            }
        }
    }

    /// Simply add the velocity of each moon to its own position. For example, if Europa has a
    /// position of x=1, y=2, z=3 and a velocity of x=-2, y=0,z=3, then its new position would be
    /// x=-1, y=2, z=6. This process does not modify the velocity of any moon.
    fn apply_velocity(&mut self) {
        for moon in self.moons.iter_mut() {
            for i in 0..DIMENSIONS {
                moon.position[i] += moon.velocity[i]
            }
        }
    }
}

fn moons_from_str(input: &str) -> Vec<Moon> {
    let mut moons = Vec::new();
    for line in input.lines() {
        let mut position = [0; DIMENSIONS];
        for (i, coordinate) in line
            .split(',')
            .take(3)
            .map(|part| parse_coordinate_from_str(part))
            .enumerate()
        {
            position[i] = coordinate
        }
        moons.push(Moon::new(position))
    }
    moons
}

fn parse_coordinate_from_str(input: &str) -> Value {
    let numeric_chars: String = input
        .chars()
        .filter(|c| c.is_digit(10) || *c == '-')
        .collect();
    numeric_chars.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_moons() {
        let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>";
        let moons = moons_from_str(input);

        let expected = vec![
            Moon {
                position: [-1, 0, 2],
                velocity: [0, 0, 0],
            },
            Moon {
                position: [2, -10, -7],
                velocity: [0, 0, 0],
            },
        ];
    }

    #[test]
    fn test_example1() {
        let moons = moons_from_str(
            "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>",
        );

        let zero_step = vec![
            Moon {
                position: [-1, 0, 2],
                velocity: [0, 0, 0],
            },
            Moon {
                position: [2, -10, -7],
                velocity: [0, 0, 0],
            },
            Moon {
                position: [4, -8, 8],
                velocity: [0, 0, 0],
            },
            Moon {
                position: [3, 5, -1],
                velocity: [0, 0, 0],
            },
        ];
        assert_eq!(moons, zero_step);

        let mut system = System::new(moons);
        let count = system.step();
        let first_step = vec![
            Moon {
                position: [2, -1, 1],
                velocity: [3, -1, -1],
            },
            Moon {
                position: [3, -7, -4],
                velocity: [1, 3, 3],
            },
            Moon {
                position: [1, -7, 5],
                velocity: [-3, 1, -3],
            },
            Moon {
                position: [2, 2, 0],
                velocity: [-1, -3, 1],
            },
        ];

        assert_eq!(1, count);
        assert_eq!(system.moons, first_step);
    }

    #[test]
    fn test_example10() {
        let moons = moons_from_str(
            "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>",
        );

        let tenth_step = vec![
            Moon {
                position: [2, 1, -3],
                velocity: [-3, -2, 1],
            },
            Moon {
                position: [1, -8, 0],
                velocity: [-1, 1, 3],
            },
            Moon {
                position: [3, -6, 1],
                velocity: [3, 2, -3],
            },
            Moon {
                position: [2, 0, 4],
                velocity: [ 1, -1, -1],
            },
        ];

        let mut system = System::new(moons);
        while system.step() < 10 {
        }

        assert_eq!(tenth_step, system.moons)
    }
}
