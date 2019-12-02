fn main() {
    println!("Hello, world!");
}

struct Module{
    mass: u32
}

impl Module {
    /// Required fuel is:
    /// 1. `mass` divided by 3
    /// 2. round down
    /// 3. subtract 2
    fn get_fuel_required(&self) -> u32 {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let module = Module{mass:12};
        assert_eq!(module.get_fuel_required(), 2)
    }

    #[test]
    fn example_2() {
        let module = Module{mass:14};
        assert_eq!(module.get_fuel_required(), 2)
    }

    #[test]
    fn example_3() {
        let module = Module{mass:1969};
        assert_eq!(module.get_fuel_required(), 654)
    }

    #[test]
    fn example_4() {
        let module = Module{mass:100756};
        assert_eq!(module.get_fuel_required(), 3358)
    }
}
