use std::collections::VecDeque;

pub type Value = i32;
type Addr = usize;

#[derive(Debug, Clone)]
pub struct Program {
    data: Vec<Value>,
    instruction_ptr: Addr,
    input: VecDeque<Value>,
    output: VecDeque<Value>,
    elapsed: usize,
}

impl Program {
    pub fn new(data: Vec<Value>) -> Self {
        Program {
            data,
            instruction_ptr: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            elapsed: 0,
        }
    }
    pub fn set_input(&mut self, value: Value) {
        self.input.push_back(value);
    }
    /// `run()` will run the Program until it halts, and return all output generated
    pub fn run(&mut self) -> &VecDeque<Value> {
        while let Some(steps) = self.execute_instruction() {
            self.instruction_ptr += steps;
            self.elapsed += 1;
        }
        &self.output
    }

    /// `run_pipe()` will pause execution after every output and return `Some(output)`
    /// When execution terminates as `OpCode::Halt` is reached, `None` is returned
    pub fn run_pipe(&mut self) -> Option<Value> {
        while let Some(steps) = self.execute_instruction() {
            self.instruction_ptr += steps;
            self.elapsed += 1;
            if ! self.output.is_empty() {
                return self.output.pop_front()
            }
        }
        // Program has halted
        None
    }
    // needed for day2 back-compatibility
    pub fn inspect(&self, position: usize) -> Value {
        self.data[position]
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
                let target_addr = self.address_at(self.instruction_ptr + 1);

                let input = self.input.pop_front().expect("Not enough input provided!");
                self.set(target_addr, input);
                Some(2)
            }
            OpCode::Output => {
                let value = self.value_at(self.instruction_ptr + 1);
                self.output.push_back(value) ;
//                                println!("Output: {}", value);
                Some(2)
            }
            OpCode::JumpIfTrue => {
                if self.param(1, instruction.parameter_modes[0]) != 0 {
                    let value = self.param(2, instruction.parameter_modes[1]);

                    // FIXME this can panic
                    self.instruction_ptr = value as usize;
                    // Don't advance if instruction_ptr was set
                    return Some(0);
                }
                return Some(3);
            }
            OpCode::JumpIfFalse => {
                if self.param(1, instruction.parameter_modes[0]) == 0 {
                    let value = self.param(2, instruction.parameter_modes[1]);

                    // FIXME this can panic
                    self.instruction_ptr = value as usize;
                    // Don't advance if instruction_ptr was set
                    return Some(0);
                }
                return Some(3);
            }
            OpCode::LessThan => {
                let target_addr = self.address_at(self.instruction_ptr + 3);
                if self.param(1, instruction.parameter_modes[0])
                    < self.param(2, instruction.parameter_modes[1])
                {
                    self.set(target_addr, 1)
                } else {
                    self.set(target_addr, 0)
                }
                Some(4)
            }
            OpCode::Equals => {
                let target_addr = self.address_at(self.instruction_ptr + 3);
                if self.param(1, instruction.parameter_modes[0])
                    == self.param(2, instruction.parameter_modes[1])
                {
                    self.set(target_addr, 1)
                } else {
                    self.set(target_addr, 0)
                }
                Some(4)
            }
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
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
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
        5 => OpCode::JumpIfTrue,
        6 => OpCode::JumpIfFalse,
        7 => OpCode::LessThan,
        8 => OpCode::Equals,
        99 => OpCode::Halt,
        _ => panic!("Oops, unrecognized opcode: {}", value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParameterMode::{Immediate, Position};

    // Day 5
    #[test]
    fn test_equals_instruction() {
        let mut position_program_eq_8 = Program::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        let mut position_program_ne_8 = position_program_eq_8.clone();
        position_program_eq_8.set_input(8);
        position_program_ne_8.set_input(1);

        let mut immediate_program_eq_8 = Program::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        let mut immediate_program_ne_8 = immediate_program_eq_8.clone();
        immediate_program_eq_8.set_input(8);
        immediate_program_ne_8.set_input(1);

        position_program_eq_8.run();
        position_program_ne_8.run();
        immediate_program_eq_8.run();
        immediate_program_ne_8.run();

        assert_eq!(position_program_eq_8.data[9], 1);
        assert_eq!(position_program_ne_8.data[9], 0);
        assert_eq!(immediate_program_eq_8.data[3], 1);
        assert_eq!(immediate_program_ne_8.data[3], 0);
    }

    #[test]
    fn test_jump_instruction() {
        let mut position_program_zero = Program::new(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        let mut position_program_nonzero = position_program_zero.clone();
        position_program_zero.set_input(0);
        position_program_nonzero.set_input(8);

        let mut immediate_program_zero =
            Program::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        let mut immediate_program_nonzero = immediate_program_zero.clone();
        immediate_program_zero.set_input(0);
        immediate_program_nonzero.set_input(8);

        position_program_zero.run();
        println!("{:?}", position_program_zero.data);
        position_program_nonzero.run();
        println!("{:?}", position_program_nonzero.data);
        immediate_program_zero.run();
        println!("{:?}", immediate_program_zero.data);
        immediate_program_nonzero.run();
        println!("{:?}", immediate_program_nonzero.data);

        assert_eq!(position_program_zero.data[13], 0);
        assert_eq!(position_program_nonzero.data[13], 1);
        assert_eq!(immediate_program_zero.data[12], 0);
        assert_eq!(immediate_program_nonzero.data[12], 1);
    }

    #[test]
    fn test_lt_instruction() {
        let position_program = Program::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        let immediate_program = Program::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
    }

    #[test]
    fn test_simple_programs() {
        let mut neg_program = Program::new(vec![1101, 100, -1, 4, 0]);
        let mut mul_program = Program::new(vec![1002, 4, 3, 4, 33]);

        let neg_out = neg_program.run();
        let mul_out = mul_program.run();

        assert_eq!(neg_program.data, [1101, 100, -1, 4, 99]);
        assert_eq!(mul_program.data, [1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_parse_parameter_modes() {
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
        let jit = 5;
        let jif = 6;
        let lt = 7;
        let eq = 8;

        let oh = opcode_from_value(h);
        let oa = opcode_from_value(a);
        let om = opcode_from_value(m);
        let oi = opcode_from_value(i);
        let oo = opcode_from_value(o);
        let ojit = opcode_from_value(jit);
        let ojif = opcode_from_value(jif);
        let olt = opcode_from_value(lt);
        let oeq = opcode_from_value(eq);

        assert_eq!(oh, OpCode::Halt);
        assert_eq!(oa, OpCode::Add);
        assert_eq!(om, OpCode::Mul);
        assert_eq!(oi, OpCode::Input);
        assert_eq!(oo, OpCode::Output);
        assert_eq!(ojit, OpCode::JumpIfTrue);
        assert_eq!(ojif, OpCode::JumpIfFalse);
        assert_eq!(olt, OpCode::LessThan);
        assert_eq!(oeq, OpCode::Equals);
    }

    // Day 2
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
