use std::collections::{HashMap, VecDeque};

pub type Value = i64;
type Addr = usize;

// TODO make Memory a HashMap
#[derive(Debug, Clone)]
pub struct Program {
    memory: HashMap<Addr, Value>,
    instruction_ptr: Addr,
    relative_base: Addr,
    input: VecDeque<Value>,
    output: VecDeque<Value>,
    elapsed: usize,
}

impl Program {
    pub fn new(data: &[Value]) -> Self {
        let memory = data.iter().cloned().enumerate().collect();
        Program {
            memory,
            instruction_ptr: 0,
            relative_base: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            elapsed: 0,
        }
    }
    pub fn set_input(&mut self, value: Value) {
        self.input.push_back(value);
    }
    // Temporary back compatibility layer, if there are high values in the data this will not work
    // and fill up your Memory
    pub fn dump_memory(&self) -> Vec<Value> {
        if self.memory.is_empty() {
            return Vec::new();
        }
        // Memory can't be empty here
        let max_address = self.memory.keys().max().unwrap();
        let mut memory = vec![0; *max_address + 1];

        for (key, value) in &self.memory {
            memory[*key] = *value;
        }
        memory
    }
    /// `run()` will run the Program until it halts, and return all output generated
    pub fn run(&mut self) -> &VecDeque<Value> {
        while let Some(steps) = self.execute_instruction() {
            self.instruction_ptr += steps;
            self.elapsed += 1;
        }
        //        println!("Steps taken: {}", self.elapsed);
        &self.output
    }

    /// `run_pipe()` will pause execution after every output and return `Some(output)`
    /// When execution terminates as `OpCode::Halt` is reached, `None` is returned
    pub fn run_pipe(&mut self) -> Option<Value> {
        while let Some(steps) = self.execute_instruction() {
            self.instruction_ptr += steps;
            self.elapsed += 1;
            if !self.output.is_empty() {
                return self.output.pop_front();
            }
        }
        // Program has halted
        None
    }
    // needed for day2 back-compatibility
    pub fn inspect(&self, position: usize) -> Value {
        let value = self
            .memory
            .get(&position)
            .expect("Inspecting unknown memory address");
        *value
    }
    fn current_instruction(&self) -> Instruction {
        let instruction = self
            .memory
            .get(&self.instruction_ptr)
            .expect("Instruction pointer at unknown memory address."); //data[self.instruction_ptr];
        instruction_from_value(*instruction)
    }
    fn execute_instruction(&mut self) -> Option<usize> {
        let instruction = self.current_instruction();
        match instruction.opcode {
            OpCode::Halt => None,
            OpCode::Add => {
                let sum = self.param(1, instruction.parameter_modes[0])
                    + self.param(2, instruction.parameter_modes[1]);
                let target_addr = self.address_at(self.instruction_ptr + 3);
                self.set(target_addr, sum, instruction.parameter_modes[2]);
                Some(4)
            }
            OpCode::Mul => {
                let product = self.param(1, instruction.parameter_modes[0])
                    * self.param(2, instruction.parameter_modes[1]);
                let target_addr = self.address_at(self.instruction_ptr + 3);
                self.set(target_addr, product, instruction.parameter_modes[2]);
                Some(4)
            }
            OpCode::Input => {
                let target_addr = self.address_at(self.instruction_ptr + 1);

                let input = self.input.pop_front().expect("Not enough input provided!");
                self.set(target_addr, input, instruction.parameter_modes[0]);
                Some(2)
            }
            OpCode::Output => {
                let value = self.param(1, instruction.parameter_modes[0]);
                self.output.push_back(value);
                // println!("Output: {}", value);
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
                Some(3)
            }
            OpCode::JumpIfFalse => {
                if self.param(1, instruction.parameter_modes[0]) == 0 {
                    let value = self.param(2, instruction.parameter_modes[1]);

                    // FIXME this can panic
                    self.instruction_ptr = value as usize;
                    // Don't advance if instruction_ptr was set
                    return Some(0);
                }
                Some(3)
            }
            OpCode::LessThan => {
                let target_addr = self.address_at(self.instruction_ptr + 3);
                if self.param(1, instruction.parameter_modes[0])
                    < self.param(2, instruction.parameter_modes[1])
                {
                    self.set(target_addr, 1, instruction.parameter_modes[2])
                } else {
                    self.set(target_addr, 0, instruction.parameter_modes[2])
                }
                Some(4)
            }
            OpCode::Equals => {
                let target_addr = self.address_at(self.instruction_ptr + 3);
                if self.param(1, instruction.parameter_modes[0])
                    == self.param(2, instruction.parameter_modes[1])
                {
                    self.set(target_addr, 1, instruction.parameter_modes[2])
                } else {
                    self.set(target_addr, 0, instruction.parameter_modes[2])
                }
                Some(4)
            }
            OpCode::SetRelativeBase => {
                let offset = self.param(1, instruction.parameter_modes[0]);
                self.relative_base = (self.relative_base as Value + offset) as usize;
                Some(2)
            }
        }
    }
    // TODO get ParameterMode(offset) instead
    fn param(&self, param_pos: usize, mode: ParameterMode) -> Value {
        if param_pos > 3 || param_pos < 1 {
            panic!("Parameters 1-3 are supported. Got: {}", param_pos)
        }
        let param_addr = self.instruction_ptr + param_pos;
        match mode {
            ParameterMode::Position => self.value_at_position(param_addr), //data[self.address_at(self.instruction_ptr + offset)],
            ParameterMode::Immediate => self.value_at(param_addr),
            ParameterMode::Relative => self.value_at_relative_position(param_addr),
        }
    }
    fn set(&mut self, addr: usize, value: Value, mode: ParameterMode) {
        let dest_addr = match mode {
            ParameterMode::Position => addr,
            ParameterMode::Immediate => panic!("Day 5 states this will never happen"),
            ParameterMode::Relative => (self.relative_base as Value + addr as Value) as usize,
        };

        self.memory.insert(dest_addr, value);
    }
    fn address_at(&self, addr: usize) -> usize {
        // FIXME this panics on invalid values
        self.memory[&addr] as usize
    }
    fn value_at(&self, addr: usize) -> Value {
        self.memory.get(&addr).cloned().unwrap_or(0)
    }
    fn value_at_position(&self, addr: usize) -> Value {
        let value_address = self.address_at(addr);
        self.memory.get(&value_address).cloned().unwrap_or(0)
        //        self.memory[self.address_at(addr)]
    }
    fn value_at_relative_position(&self, addr: usize) -> Value {
        let offset = self.value_at(addr);
        let addr = (self.relative_base as Value + offset) as usize;
        self.memory.get(&addr).cloned().unwrap_or(0)
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
    SetRelativeBase,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

type ParameterModes = [ParameterMode; 3];

#[derive(Clone, Debug)]
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
    for parameter_mode in &mut parameter_modes {
        let digit = remainder % 10;
        if digit == 2 {
            *parameter_mode = ParameterMode::Relative
        } else if digit == 1 {
            *parameter_mode = ParameterMode::Immediate
        } else if digit == 0 {
            // nothing to do
        } else {
            panic!("Unknown parameter mode: {}", digit)
        }

        remainder /= 10;
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
        9 => OpCode::SetRelativeBase,
        99 => OpCode::Halt,
        _ => panic!("Oops, unrecognized opcode: {}", value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParameterMode::{Immediate, Position, Relative};

    // Day 9 - relative base

    #[test]
    fn test_implementation_correctness() {
        let cases = vec![
            (vec![109, -1, 4, 1, 99], -1),
            (vec![109, -1, 104, 1, 99], 1),
            (vec![109, -1, 204, 1, 99], 109),
            (vec![109, 1, 9, 2, 204, -6, 99], 204),
            (vec![109, 1, 109, 9, 204, -6, 99], 204),
            (vec![109, 1, 209, -1, 204, -106, 99], 204),
        ];

        for (data, expected) in cases {
            let mut p = Program::new(&data);
            let output = p.run();
            assert_eq!(output[0], expected)
        }
    }
    #[test]
    fn test_implementation_correctness_with_input() {
        let inputs: Vec<i64> = vec![0, 1, 100, i32::max_value() as i64 + 1];
        let test_data = vec![
            vec![109, 1, 3, 3, 204, 2, 99],
            vec![109, 1, 203, 2, 204, 2, 99],
        ];
        for input in inputs {
            for data in &test_data {
                let mut p = Program::new(&data);
                p.set_input(input);
                let output = p.run();
                assert_eq!(output[0], input)
            }
        }
    }
    #[test]
    fn test_init_memory() {
        let data = vec![0, 1, 2, 3];
        let p = Program::new(&data);

        let mut expected: HashMap<Addr, Value> = HashMap::new();
        expected.insert(0, 0);
        expected.insert(1, 1);
        expected.insert(2, 2);
        expected.insert(3, 3);

        assert_eq!(expected, p.memory);
    }
    #[test]
    fn test_set_relative_base_instruction() {
        // Quine
        let data = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut program = Program::new(&data);
        let output = program.run();

        assert_eq!(output, &data);
    }

    #[test]
    fn test_set_relative_base_16digit_number() {
        let mut program = Program::new(&vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        let output = program.run();

        assert_eq!(output[0], 1219070632396864);
    }

    #[test]
    fn test_set_relative_base_output() {
        let data = vec![104, 1125899906842624, 99];

        let mut program = Program::new(&data);
        let output = program.run();

        assert_eq!(data[1], output[0])
    }

    // Day 5
    #[test]
    fn test_equals_instruction() {
        let mut position_program_eq_8 = Program::new(&vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        let mut position_program_ne_8 = position_program_eq_8.clone();
        position_program_eq_8.set_input(8);
        position_program_ne_8.set_input(1);

        let mut immediate_program_eq_8 = Program::new(&vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        let mut immediate_program_ne_8 = immediate_program_eq_8.clone();
        immediate_program_eq_8.set_input(8);
        immediate_program_ne_8.set_input(1);

        position_program_eq_8.run();
        position_program_ne_8.run();
        immediate_program_eq_8.run();
        immediate_program_ne_8.run();

        assert_eq!(position_program_eq_8.inspect(9), 1);
        assert_eq!(position_program_ne_8.inspect(9), 0);
        assert_eq!(immediate_program_eq_8.inspect(3), 1);
        assert_eq!(immediate_program_ne_8.inspect(3), 0);
    }

    #[test]
    fn test_jump_instruction() {
        let mut position_program_zero = Program::new(&vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        let mut position_program_nonzero = position_program_zero.clone();
        position_program_zero.set_input(0);
        position_program_nonzero.set_input(8);

        let mut immediate_program_zero =
            Program::new(&vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        let mut immediate_program_nonzero = immediate_program_zero.clone();
        immediate_program_zero.set_input(0);
        immediate_program_nonzero.set_input(8);

        position_program_zero.run();
        position_program_nonzero.run();
        immediate_program_zero.run();
        immediate_program_nonzero.run();

        assert_eq!(position_program_zero.inspect(13), 0);
        assert_eq!(position_program_nonzero.inspect(13), 1);
        assert_eq!(immediate_program_zero.inspect(12), 0);
        assert_eq!(immediate_program_nonzero.inspect(12), 1);
    }

    #[test]
    fn test_simple_programs() {
        let mut neg_program = Program::new(&vec![1101, 100, -1, 4, 0]);
        let mut mul_program = Program::new(&vec![1002, 4, 3, 4, 33]);

        let neg_out = neg_program.run();
        let mul_out = mul_program.run();

        assert_eq!(neg_program.dump_memory(), [1101, 100, -1, 4, 99]);
        assert_eq!(mul_program.dump_memory(), [1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_parse_parameter_modes() {
        let p1 = 111;
        let p2 = 0;
        let p3 = 011;
        let p4 = 1;
        let p5 = 010;
        let p6 = 101;
        let p7 = 210;
        let p8 = 2;

        let pms1 = parameter_mode_from_value(p1);
        let pms2 = parameter_mode_from_value(p2);
        let pms3 = parameter_mode_from_value(p3);
        let pms4 = parameter_mode_from_value(p4);
        let pms5 = parameter_mode_from_value(p5);
        let pms6 = parameter_mode_from_value(p6);
        let pms7 = parameter_mode_from_value(p7);
        let pms8 = parameter_mode_from_value(p8);

        assert_eq!([Immediate; 3], pms1);
        assert_eq!([Position; 3], pms2);
        assert_eq!([Immediate, Immediate, Position], pms3);
        assert_eq!([Immediate, Position, Position], pms4);
        assert_eq!([Position, Immediate, Position], pms5);
        assert_eq!([Immediate, Position, Immediate], pms6);
        assert_eq!([Position, Immediate, Relative], pms7);
        assert_eq!([Relative, Position, Position], pms8);
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
        let srb = 9;

        let oh = opcode_from_value(h);
        let oa = opcode_from_value(a);
        let om = opcode_from_value(m);
        let oi = opcode_from_value(i);
        let oo = opcode_from_value(o);
        let ojit = opcode_from_value(jit);
        let ojif = opcode_from_value(jif);
        let olt = opcode_from_value(lt);
        let oeq = opcode_from_value(eq);
        let osrb = opcode_from_value(srb);

        assert_eq!(oh, OpCode::Halt);
        assert_eq!(oa, OpCode::Add);
        assert_eq!(om, OpCode::Mul);
        assert_eq!(oi, OpCode::Input);
        assert_eq!(oo, OpCode::Output);
        assert_eq!(ojit, OpCode::JumpIfTrue);
        assert_eq!(ojif, OpCode::JumpIfFalse);
        assert_eq!(olt, OpCode::LessThan);
        assert_eq!(oeq, OpCode::Equals);
        assert_eq!(osrb, OpCode::SetRelativeBase);
    }

    // Day 2
    #[test]
    fn example_1() {
        let mut p = Program::new(&vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        p.run();
        //        assert_eq!(p.data, [1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        //        p.run();
        assert_eq!(
            p.dump_memory(),
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        )
    }
    #[test]
    fn example_2() {
        let mut p = Program::new(&vec![1, 0, 0, 0, 99]);
        p.run();
        assert_eq!(p.dump_memory(), [2, 0, 0, 0, 99]);
    }
    #[test]
    fn example_3() {
        let mut p = Program::new(&vec![1, 0, 0, 0, 99]);
        p.run();
        assert_eq!(p.dump_memory(), [2, 0, 0, 0, 99]);
    }
    #[test]
    fn example_4() {
        let mut p = Program::new(&vec![2, 3, 0, 3, 99]);
        p.run();
        assert_eq!(p.dump_memory(), [2, 3, 0, 6, 99]);
    }
    #[test]
    fn example_5() {
        let mut p = Program::new(&vec![2, 4, 4, 5, 99, 0]);
        p.run();
        assert_eq!(p.dump_memory(), [2, 4, 4, 5, 99, 9801]);
    }
    #[test]
    fn example_6() {
        let mut p = Program::new(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        p.run();
        assert_eq!(p.dump_memory(), [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
