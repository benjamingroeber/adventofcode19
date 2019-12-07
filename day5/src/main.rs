//add two new instructions:
//
//Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. For example, the instruction 3,50 would take an input value and store it at address 50.
//Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50.

fn main() {
    let input: Vec<Value> = vec![
        3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1002, 114, 46, 224, 1001, 224, -736, 224,
        4, 224, 1002, 223, 8, 223, 1001, 224, 3, 224, 1, 223, 224, 223, 1, 166, 195, 224, 1001,
        224, -137, 224, 4, 224, 102, 8, 223, 223, 101, 5, 224, 224, 1, 223, 224, 223, 1001, 169,
        83, 224, 1001, 224, -90, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 2, 224, 1, 224, 223,
        223, 101, 44, 117, 224, 101, -131, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 5, 224, 224,
        1, 224, 223, 223, 1101, 80, 17, 225, 1101, 56, 51, 225, 1101, 78, 89, 225, 1102, 48, 16,
        225, 1101, 87, 78, 225, 1102, 34, 33, 224, 101, -1122, 224, 224, 4, 224, 1002, 223, 8, 223,
        101, 7, 224, 224, 1, 223, 224, 223, 1101, 66, 53, 224, 101, -119, 224, 224, 4, 224, 102, 8,
        223, 223, 1001, 224, 5, 224, 1, 223, 224, 223, 1102, 51, 49, 225, 1101, 7, 15, 225, 2, 110,
        106, 224, 1001, 224, -4539, 224, 4, 224, 102, 8, 223, 223, 101, 3, 224, 224, 1, 223, 224,
        223, 1102, 88, 78, 225, 102, 78, 101, 224, 101, -6240, 224, 224, 4, 224, 1002, 223, 8, 223,
        101, 5, 224, 224, 1, 224, 223, 223, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1,
        99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274,
        1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0,
        1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0,
        1105, 1, 99999, 1107, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 329, 101, 1, 223, 223,
        1108, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 344, 101, 1, 223, 223, 8, 226, 677, 224,
        102, 2, 223, 223, 1006, 224, 359, 1001, 223, 1, 223, 1007, 226, 677, 224, 1002, 223, 2,
        223, 1005, 224, 374, 101, 1, 223, 223, 1008, 677, 677, 224, 1002, 223, 2, 223, 1005, 224,
        389, 1001, 223, 1, 223, 1108, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 404, 1001, 223,
        1, 223, 1007, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 419, 1001, 223, 1, 223, 1107,
        677, 226, 224, 1002, 223, 2, 223, 1006, 224, 434, 101, 1, 223, 223, 108, 677, 677, 224,
        1002, 223, 2, 223, 1005, 224, 449, 1001, 223, 1, 223, 1107, 677, 677, 224, 102, 2, 223,
        223, 1005, 224, 464, 1001, 223, 1, 223, 108, 226, 226, 224, 1002, 223, 2, 223, 1006, 224,
        479, 1001, 223, 1, 223, 1008, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 494, 101, 1, 223,
        223, 108, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 509, 1001, 223, 1, 223, 8, 677, 226,
        224, 1002, 223, 2, 223, 1006, 224, 524, 101, 1, 223, 223, 7, 226, 677, 224, 1002, 223, 2,
        223, 1006, 224, 539, 101, 1, 223, 223, 7, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 554,
        1001, 223, 1, 223, 7, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 569, 101, 1, 223, 223,
        107, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 584, 101, 1, 223, 223, 1108, 677, 677,
        224, 102, 2, 223, 223, 1006, 224, 599, 1001, 223, 1, 223, 1008, 677, 226, 224, 1002, 223,
        2, 223, 1005, 224, 614, 1001, 223, 1, 223, 8, 677, 677, 224, 1002, 223, 2, 223, 1006, 224,
        629, 1001, 223, 1, 223, 107, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 644, 101, 1, 223,
        223, 1007, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 659, 101, 1, 223, 223, 107, 226,
        226, 224, 1002, 223, 2, 223, 1006, 224, 674, 1001, 223, 1, 223, 4, 223, 99, 226,
    ];

    let mut program = Program::new(input);

    program.run();
}

type Value = i32;
type Addr = usize;

struct Program {
    data: Vec<Value>,
    instruction_ptr: Addr,
}

impl Program {
    fn new(data: Vec<Value>) -> Self {
        Program {
            data,
            instruction_ptr: 0,
        }
    }
    fn current_instruction(&self) -> Instruction {
        let instruction = self.data[self.instruction_ptr];
        instruction_from_value(instruction)
    }
    fn execute_instruction(&mut self) -> Option<usize> {
        let instruction = self.current_instruction();
        match instruction.opcode {
            OpCode::Halt => None,
            OpCode::Add => {
                let sum = self.param(1, instruction.parameter_modes[0])
                    + self.param(2, instruction.parameter_modes[1]);
                let target_addr = self.address_at(self.instruction_ptr + 3);
                self.set(target_addr, sum);
                Some(4)
            }
            OpCode::Mul => {
                let product = self.param(1, instruction.parameter_modes[0])
                    * self.param(2, instruction.parameter_modes[1]);
                let target_addr = self.address_at(self.instruction_ptr + 3);
                self.set(target_addr, product);
                Some(4)
            }
            OpCode::Input => {
                // TODO Get actual input
                let input = 1;
                let target_addr = self.address_at(self.instruction_ptr + 1);

                self.set(target_addr, input);
                Some(2)
            }
            OpCode::Output => {
                let value = self.value_at(self.instruction_ptr + 1);
                println!("Output: {}", value);
                Some(2)
            }
        }
    }
    fn run(&mut self) {
        while let Some(steps) = self.execute_instruction() {
            self.instruction_ptr += steps
        }
    }
    fn param(&self, offset: usize, mode: ParameterMode) -> Value {
        if offset > 3 || offset < 1 {
            panic!("Parameters 1-3 are supported. Got: {}", offset)
        }
        match mode {
            ParameterMode::Position => self.data[self.address_at(self.instruction_ptr + offset)],
            ParameterMode::Immediate => self.data[self.instruction_ptr + offset],
        }
    }
    fn address_at(&self, addr: usize) -> usize {
        // FIXME this panics on invalid values
        self.data[addr] as usize
    }
    fn value_at(&self, addr: usize) -> Value {
        self.data[self.address_at(addr)]
    }
    fn set(&mut self, addr: usize, value: Value) {
        self.data[addr] = value
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum OpCode {
    Halt,
    Add,
    Mul,
    Input,
    Output,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

type ParameterModes = [ParameterMode; 3];

struct Instruction {
    opcode: OpCode,
    parameter_modes: ParameterModes,
}

fn instruction_from_value(value: Value) -> Instruction {
    let opcode = opcode_from_value(value % 100);
    let remainder = value / 100;
    let parameter_modes = parameter_mode_from_value(remainder);
    Instruction {
        opcode,
        parameter_modes,
    }
}

fn parameter_mode_from_value(value: Value) -> ParameterModes {
    let mut remainder = value;
    let mut parameter_modes = [ParameterMode::Position; 3];
    for i in 0..parameter_modes.len() {
        let digit = remainder % 10;
        if digit == 1 {
            parameter_modes[i] = ParameterMode::Immediate
        } else if digit == 0 {
            // nothing to do
        } else {
            panic!("Unknown parameter mode: {}", digit)
        }

        remainder = remainder / 10;
    }
    parameter_modes
}

fn opcode_from_value(value: Value) -> OpCode {
    match value {
        1 => OpCode::Add,
        2 => OpCode::Mul,
        3 => OpCode::Input,
        4 => OpCode::Output,
        99 => OpCode::Halt,
        _ => panic!("Oops, unrecognized opcode: {}", value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParameterMode::{Immediate, Position};

    #[test]
    fn test_simple_programs() {

        let mut neg_program = Program::new(vec![1101, 100, -1, 4, 0]);
        let mut mul_program = Program::new(vec![1002, 4, 3, 4, 33]);

        neg_program.run();
        mul_program.run();

        assert_eq!(neg_program.data, [1101, 100, -1, 4, 99]);
        assert_eq!(mul_program.data, [1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_parse_parametermodes() {
        let p1 = 111;
        let p2 = 0;
        let p3 = 011;
        let p4 = 1;
        let p5 = 010;
        let p6 = 101;

        let pms1 = parameter_mode_from_value(p1);
        let pms2 = parameter_mode_from_value(p2);
        let pms3 = parameter_mode_from_value(p3);
        let pms4 = parameter_mode_from_value(p4);
        let pms5 = parameter_mode_from_value(p5);
        let pms6 = parameter_mode_from_value(p6);

        assert_eq!([Immediate; 3], pms1);
        assert_eq!([Position; 3], pms2);
        assert_eq!([Immediate, Immediate, Position], pms3);
        assert_eq!([Immediate, Position, Position], pms4);
        assert_eq!([Position, Immediate, Position], pms5);
        assert_eq!([Immediate, Position, Immediate], pms6);
    }

    #[test]
    fn test_parse_opcode1() {
        let h = 99;
        let a = 1;
        let m = 2;
        let i = 3;
        let o = 4;

        let oh = opcode_from_value(h);
        let oa = opcode_from_value(a);
        let om = opcode_from_value(m);
        let oi = opcode_from_value(i);
        let oo = opcode_from_value(o);

        assert_eq!(oh, OpCode::Halt);
        assert_eq!(oa, OpCode::Add);
        assert_eq!(om, OpCode::Mul);
        assert_eq!(oi, OpCode::Input);
        assert_eq!(oo, OpCode::Output);
    }

    // Old tests from day 2
    #[test]
    fn example_1() {
        let mut p = Program::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        p.run();
        //        assert_eq!(p.data, [1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        //        p.run();
        assert_eq!(p.data, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50])
    }
    #[test]
    fn example_2() {
        let mut p = Program::new(vec![1, 0, 0, 0, 99]);
        p.run();
        assert_eq!(p.data, [2, 0, 0, 0, 99]);
    }
    #[test]
    fn example_3() {
        let mut p = Program::new(vec![1, 0, 0, 0, 99]);
        p.run();
        assert_eq!(p.data, [2, 0, 0, 0, 99]);
    }
    #[test]
    fn example_4() {
        let mut p = Program::new(vec![2, 3, 0, 3, 99]);
        p.run();
        assert_eq!(p.data, [2, 3, 0, 6, 99]);
    }
    #[test]
    fn example_5() {
        let mut p = Program::new(vec![2, 4, 4, 5, 99, 0]);
        p.run();
        assert_eq!(p.data, [2, 4, 4, 5, 99, 9801]);
    }
    #[test]
    fn example_6() {
        let mut p = Program::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        p.run();
        assert_eq!(p.data, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
