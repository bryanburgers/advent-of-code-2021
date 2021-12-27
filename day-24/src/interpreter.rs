use crate::program::{Instruction, Parameter, Program, Variable};

pub struct Interpreter(Program);

impl Interpreter {
    pub fn build(program: &Program) -> Self {
        Self(program.clone())
    }

    fn program(&self) -> &Program {
        &self.0
    }

    pub fn run(&self, inputs: &[i32]) -> (i32, i32, i32, i32) {
        let mut state = InterpreterState::new(inputs);

        for instruction in self.program().instructions() {
            state.apply_instruction(*instruction);
        }

        state.into_output()
    }
}

struct InterpreterState<'a> {
    inputs: &'a [i32],
    w: i32,
    x: i32,
    y: i32,
    z: i32,
    input_index: usize,
}

impl<'a> InterpreterState<'a> {
    pub fn new(inputs: &'a [i32]) -> Self {
        Self {
            inputs,
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            input_index: 0,
        }
    }

    pub fn get(&self, variable: Variable) -> i32 {
        match variable {
            Variable::W => self.w,
            Variable::X => self.x,
            Variable::Y => self.y,
            Variable::Z => self.z,
        }
    }

    pub fn get_parameter(&self, parameter: Parameter) -> i32 {
        match parameter {
            Parameter::Number(n) => n,
            Parameter::Variable(v) => self.get(v),
        }
    }

    pub fn set(&mut self, variable: Variable, value: i32) {
        match variable {
            Variable::W => {
                self.w = value;
            }
            Variable::X => {
                self.x = value;
            }
            Variable::Y => {
                self.y = value;
            }
            Variable::Z => {
                self.z = value;
            }
        }
    }

    pub fn input(&mut self) -> i32 {
        let r = self.inputs[self.input_index];
        self.input_index += 1;
        r
    }

    pub fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Inp(var) => {
                let val = self.input();
                self.set(var, val);
            }
            Instruction::Add(var, parameter) => {
                let mut val = self.get(var);
                val += self.get_parameter(parameter);
                self.set(var, val);
            }
            Instruction::Mul(var, parameter) => {
                let mut val = self.get(var);
                val *= self.get_parameter(parameter);
                self.set(var, val);
            }
            Instruction::Div(var, parameter) => {
                let mut val = self.get(var);
                val /= self.get_parameter(parameter);
                self.set(var, val);
            }
            Instruction::Mod(var, parameter) => {
                let mut val = self.get(var);
                val %= self.get_parameter(parameter);
                self.set(var, val);
            }
            Instruction::Eql(var, parameter) => {
                let val1 = self.get(var);
                let val2 = self.get_parameter(parameter);
                if val1 == val2 {
                    self.set(var, 1);
                } else {
                    self.set(var, 0);
                }
            }
        }
    }

    pub fn into_output(self) -> (i32, i32, i32, i32) {
        (self.w, self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let program = input.parse().unwrap();
        let interpreter = Interpreter::build(&program);
        assert_eq!(interpreter.run(&[0b0000]), (0, 0, 0, 0));
        assert_eq!(interpreter.run(&[0b0001]), (0, 0, 0, 1));
        assert_eq!(interpreter.run(&[0b0010]), (0, 0, 1, 0));
        assert_eq!(interpreter.run(&[0b0011]), (0, 0, 1, 1));
        assert_eq!(interpreter.run(&[0b0100]), (0, 1, 0, 0));
        assert_eq!(interpreter.run(&[0b1000]), (1, 0, 0, 0));
        assert_eq!(interpreter.run(&[0b1111]), (1, 1, 1, 1));
    }
}
