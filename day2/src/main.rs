fn main() {
    let input = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 9, 19, 1, 10, 19, 23, 2, 9, 23, 27,
        1, 6, 27, 31, 2, 31, 9, 35, 1, 5, 35, 39, 1, 10, 39, 43, 1, 10, 43, 47, 2, 13, 47, 51, 1,
        10, 51, 55, 2, 55, 10, 59, 1, 9, 59, 63, 2, 6, 63, 67, 1, 5, 67, 71, 1, 71, 5, 75, 1, 5,
        75, 79, 2, 79, 13, 83, 1, 83, 5, 87, 2, 6, 87, 91, 1, 5, 91, 95, 1, 95, 9, 99, 1, 99, 6,
        103, 1, 103, 13, 107, 1, 107, 5, 111, 2, 111, 13, 115, 1, 115, 6, 119, 1, 6, 119, 123, 2,
        123, 13, 127, 1, 10, 127, 131, 1, 131, 2, 135, 1, 135, 5, 0, 99, 2, 14, 0, 0,
    ];

    // first addresses can't exceed length of data
    for verb in 0..input.len() {
        for noun in 0..input.len() {
            let mut data = input.clone();
            data[1] = noun;
            data[2] = verb;
            let mut p = Programm::new(data);
            p.run();

            let result = p.data[0];
            // before running the programm replace position 1 with the value 12
            // and replace position 2 with the value 2
            if noun == 12 && verb == 2 {
                println!("Solution1: {}", result)
            }

            // Find the input noun and verb that cause the program to produce the output 19690720.
            // What is 100 * noun + verb?
            if p.data[0] == 19690720 {
                println!(
                    "Solution2: {}, Result: {}, Noun: {}, Verb: {}",
                    100 * noun + verb,
                    result,
                    noun,
                    verb
                )
            }
        }
    }
}

struct Programm {
    data: Vec<usize>,
    instruction_ptr: usize,
}

impl Programm {
    fn new(data: Vec<usize>) -> Self {
        Programm {
            data,
            instruction_ptr: 0,
        }
    }
    fn run(&mut self) {
        loop {
            match opcode_from_usize(self.data[self.instruction_ptr]) {
                OpCode::Halt => break,
                OpCode::Add => {
                    self.set_target(self.val_at(1) + self.val_at(2));
                }
                OpCode::Mul => {
                    self.set_target(self.val_at(1) * self.val_at(2));
                }
            }
            // Valid for both Add and Mul, Halt never reaches this
            self.instruction_ptr += 4
        }
    }
    fn param(&self, offset: usize) -> usize {
        if offset < 1 || offset > 3 {
            panic! {"Only parameters from 1 - 3 are supported"}
        }
        self.instruction_ptr + offset
    }
    fn addr_at(&self, offset: usize) -> usize {
        let param = self.param(offset);
        self.data[param]
    }
    fn val_at(&self, offset: usize) -> usize {
        self.data[self.addr_at(offset)]
    }
    fn set_target(&mut self, value: usize) {
        let target_addr = self.addr_at(3);
        self.data[target_addr] = value
    }
}

enum OpCode {
    Halt,
    Add,
    Mul,
}

fn opcode_from_usize(code: usize) -> OpCode {
    match code {
        1 => OpCode::Add,
        2 => OpCode::Mul,
        99 => OpCode::Halt,
        _ => panic!("Oops, unrecognized opcode: {}", code),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut p = Programm::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        p.run();
        //        assert_eq!(p.data, [1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        //        p.run();
        assert_eq!(p.data, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50])
    }
    #[test]
    fn example_2() {
        let mut p = Programm::new(vec![1, 0, 0, 0, 99]);
        p.run();
        assert_eq!(p.data, [2, 0, 0, 0, 99]);
    }
    #[test]
    fn example_3() {
        let mut p = Programm::new(vec![1, 0, 0, 0, 99]);
        p.run();
        assert_eq!(p.data, [2, 0, 0, 0, 99]);
    }
    #[test]
    fn example_4() {
        let mut p = Programm::new(vec![2, 3, 0, 3, 99]);
        p.run();
        assert_eq!(p.data, [2, 3, 0, 6, 99]);
    }
    #[test]
    fn example_5() {
        let mut p = Programm::new(vec![2, 4, 4, 5, 99, 0]);
        p.run();
        assert_eq!(p.data, [2, 4, 4, 5, 99, 9801]);
    }
    #[test]
    fn example_6() {
        let mut p = Programm::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        p.run();
        assert_eq!(p.data, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
