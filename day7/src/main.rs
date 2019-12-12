use intcode::{Program, Value};
use permutohedron::Heap;

fn main() {
    let data = vec![3,8,1001,8,10,8,105,1,0,0,21,42,67,88,105,114,195,276,357,438,99999,3,9,101,4,9,9,102,3,9,9,1001,9,2,9,102,4,9,9,4,9,99,3,9,1001,9,4,9,102,4,9,9,101,2,9,9,1002,9,5,9,1001,9,2,9,4,9,99,3,9,1001,9,4,9,1002,9,4,9,101,2,9,9,1002,9,2,9,4,9,99,3,9,101,4,9,9,102,3,9,9,1001,9,5,9,4,9,99,3,9,102,5,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,99];
    let amplifier = Amplifier::new(data);

    // Part 1
    let mut possible_values = vec![0, 1, 2, 3, 4];
    let permutations = Heap::new(&mut possible_values);

    let max = permutations.into_iter().map(|p| amplifier.amplify(&p)).max();
    println!("Highest possible thrust input is: {:?}", max);

    // Part 2
    let mut possible_values = vec![5, 6, 7, 8, 9];
    let permutations = Heap::new(&mut possible_values);

    let max = permutations.into_iter().map(|p| amplifier.amplify_pipe(&p)).max();
    println!("Highest possible thrust input is: {:?}", max)

}

struct Amplifier {
    data: Vec<Value>,
}

impl Amplifier {
    fn new(data: Vec<Value>) -> Self {
        Amplifier{data}
    }
    fn amplify(&self, phases: &[Value]) -> Value {
        let mut input = 0;
        for i in 0..phases.len() {
            let mut p = Program::new(self.data.clone());
            let phase = phases[i];

            p.set_input(phase);
            p.set_input(input);
            let output = p.run();
            if output.len() > 1 {
                panic!("Got more than one line of output: {:?}", output)
            }
            input = output[0]
        }
        input
    }
    fn amplify_pipe(&self, phases: &[Value]) -> Value {
        let mut programs = vec![Program::new(self.data.clone()); phases.len()];
        // set phase values once
        for i in 0..phases.len() {
            programs[i].set_input(phases[i]);
        }

        // set input for first Amplifier once
        let mut input = 0;
        loop {
            for i in 0..programs.len() {
                let p = &mut programs[i];
                p.set_input(input);

                // loop until program halts
                if let Some(output) = p.run_pipe() {
                    input = output
                } else {
                    return input
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1_p1() {
        let data = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let a = Amplifier::new(data);

        let value = a.amplify(&vec![4,3,2,1,0]);
        assert_eq!(value, 43210)
    }

    #[test]
    fn test_example2_p1() {
        let data = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        let a = Amplifier::new(data);

        let value = a.amplify(&vec![0,1,2,3,4]);
        assert_eq!(value, 54321)
    }

    #[test]
    fn test_example3_p1() {
        let data = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        let a = Amplifier::new(data);

        let value = a.amplify(&vec![1,0,4,3,2]);
        assert_eq!(value, 65210)
    }

    #[test]
    fn test_example1_p2() {
        let data = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        let a = Amplifier::new(data);

        let value = a.amplify_pipe(&vec![9,8,7,6,5]);
        assert_eq!(value, 139629729)
    }

    #[test]
    fn test_example2_p2() {
        let data = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                        -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                        53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
        let a = Amplifier::new(data);

        let value = a.amplify_pipe(&vec![9,7,8,5,6]);
        assert_eq!(value, 18216)
    }
}