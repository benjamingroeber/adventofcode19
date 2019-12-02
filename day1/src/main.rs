use std::io::{stdin, BufRead};
use std::fmt::Error;

/// The answer is the sum of all modules
fn main() -> Result<(), Box<Error>> {
    let mut sum= 0;
    let mut full_sum = 0;
    for line in stdin().lock().lines() {
        let mass = line.unwrap().parse().unwrap();

        let module = Module{mass};

        sum += module.get_fuel_required();
        full_sum += module.get_full_fuel_required();
    }

    println!("The required fuel for all modules is: {}", sum);
    println!("The full required fuel for all modules is: {}", full_sum);

    Ok(())
}

struct Module{
    mass: u64
}

impl Module {

    /// this considers only the mass of the module
    fn get_fuel_required(&self) -> u64 {
        fuel_required_for_mass(self.mass)
    }

    /// this considers the mass of the module
    /// plus the additional mass of the additional fuel
    /// this will loop endlessly if fuel_required_for_mass doesn't decrease
    fn get_full_fuel_required(&self) -> u64 {
        let mut additional_fuel = self.get_fuel_required();
        let mut sum = additional_fuel;
        while additional_fuel > 0 {
            additional_fuel = fuel_required_for_mass(additional_fuel);
            sum += additional_fuel
        }

        sum
    }
}

/// Required fuel is:
/// 1. `mass` divided by 3
/// 2. round down
/// 3. subtract 2
fn fuel_required_for_mass(mass: u64) -> u64 {
    // integer division already loses the Mantissa
    let quotient = mass / 3;
    if quotient > 1 {
        quotient - 2
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_11() {
        assert_eq!(fuel_required_for_mass(12), 2)
    }

    #[test]
    fn example_12() {
        assert_eq!(fuel_required_for_mass(14), 2)
    }

    #[test]
    fn example_13() {
        assert_eq!(fuel_required_for_mass(1_969), 654)
    }

    #[test]
    fn example_14() {
        assert_eq!(fuel_required_for_mass(100_756), 33_583)
    }

    #[test]
    fn example_21() {
        let module = Module{mass:12};
        assert_eq!(module.get_full_fuel_required(), 2)
    }

    #[test]
    fn example_22() {
        let module = Module{mass:1_969};
        assert_eq!(module.get_full_fuel_required(), 966)
    }

    #[test]
    fn example_23() {
        let module = Module{mass: 100_756};
        assert_eq!(module.get_full_fuel_required(), 50_346)
    }
}
