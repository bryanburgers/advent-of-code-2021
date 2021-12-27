use crate::program::{Instruction, Parameter, Program, Variable};

pub struct Compiler(());

impl Compiler {
    pub fn new() -> Self {
        Self(())
    }

    fn variable_name(&self, variable: Variable) -> &str {
        match variable {
            Variable::W => "$w",
            Variable::X => "$x",
            Variable::Y => "$y",
            Variable::Z => "$z",
        }
    }

    fn push_binary(&self, program: &mut String, op: &str, var1: Variable, parameter: Parameter) {
        match parameter {
            Parameter::Number(val) => program.push_str(
                format!(
                    r#"
            local.get {var1}
            i32.const {val}
            {op}
            local.set {var1}
                        "#,
                    op = op,
                    var1 = self.variable_name(var1),
                    val = val,
                )
                .trim_end(),
            ),
            Parameter::Variable(var2) => program.push_str(
                format!(
                    r#"
            local.get {var1}
            local.get {var2}
            {op}
            local.set {var1}
                        "#,
                    op = op,
                    var1 = self.variable_name(var1),
                    var2 = self.variable_name(var2),
                )
                .trim_end(),
            ),
        }
    }

    pub fn compile(&self, program: &Program) -> String {
        let input = program;
        let mut program = String::new();

        for instruction in &input.0 {
            program.push_str(&format!("\n\n        ;; {}\n", instruction));
            match *instruction {
                Instruction::Inp(var) => {
                    program.push_str(
                        format!(
                            r#"
            ;; Load the i32 at $input_offset
            local.get $input_offset
            i32.load
            local.set {var}
            ;; Load the i32 at $input_offset again and increment it by 4 (sizeof i32)
            local.get $input_offset
            i32.const 4
            i32.add
            local.set $input_offset
                        "#,
                            var = self.variable_name(var),
                        )
                        .trim_end(),
                    );
                }
                Instruction::Add(var1, parameter) => {
                    self.push_binary(&mut program, "i32.add", var1, parameter)
                }
                Instruction::Mul(var1, parameter) => {
                    self.push_binary(&mut program, "i32.mul", var1, parameter)
                }
                Instruction::Div(var1, parameter) => {
                    self.push_binary(&mut program, "i32.div_s", var1, parameter)
                }
                Instruction::Mod(var1, parameter) => {
                    self.push_binary(&mut program, "i32.rem_s", var1, parameter)
                }
                Instruction::Eql(var1, parameter) => {
                    self.push_binary(&mut program, "i32.eq", var1, parameter)
                }
            }
        }

        format!(
            r#"(module
    (import "host" "mem" (memory 1))
    (func (export "run") (param $input_len i32) (result i32 i32 i32 i32) (local $input_offset i32) (local $w i32) (local $x i32) (local $y i32) (local $z i32) 
        ;; Set all of the locals
        i32.const 0
        local.set $input_offset
        i32.const 0
        local.set $w
        i32.const 0
        local.set $x
        i32.const 0
        local.set $y
        i32.const 0
        local.set $z

        ;; Program
        {program}

        ;; Return
        local.get $w
        local.get $x
        local.get $y
        local.get $z
    )
)"#,
            program = program
        )
    }
}

pub struct Runner {
    store: wasmtime::Store<()>,
    memory: wasmtime::Memory,
    typed_func: wasmtime::TypedFunc<i32, (i32, i32, i32, i32)>,
}

impl Runner {
    pub fn build(program: &Program) -> Self {
        let compiler = Compiler::new();
        let wast = compiler.compile(program);

        let engine = wasmtime::Engine::default();
        let module = wasmtime::Module::new(&engine, wast).unwrap();
        let mut linker = wasmtime::Linker::new(&engine);
        let mut store = wasmtime::Store::new(&engine, ());
        let memory =
            wasmtime::Memory::new(&mut store, wasmtime::MemoryType::new(1, Some(1))).unwrap();
        linker.define("host", "mem", memory).unwrap();
        let instance = linker.instantiate(&mut store, &module).unwrap();
        let typed_func = instance
            .get_typed_func::<i32, (i32, i32, i32, i32), _>(&mut store, "run")
            .unwrap();

        Self {
            store,
            memory,
            typed_func,
        }
    }
    pub fn run(&mut self, inputs: &[i32]) -> (i32, i32, i32, i32) {
        for (idx, v) in inputs.iter().enumerate() {
            self.memory
                .write(&mut self.store, idx * 4, &v.to_le_bytes())
                .unwrap();
        }
        self.typed_func
            .call(&mut self.store, inputs.len() as i32)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_program() {
        let input = include_str!("example.txt");
        let program = input.parse().unwrap();
        let compiler = Compiler::new();
        let wast = compiler.compile(&program);
        println!("{}", wast);

        let engine = wasmtime::Engine::default();
        let module = wasmtime::Module::new(&engine, wast).unwrap();
        let mut linker = wasmtime::Linker::new(&engine);
        let mut store = wasmtime::Store::new(&engine, 0);
        let memory =
            wasmtime::Memory::new(&mut store, wasmtime::MemoryType::new(1, Some(1))).unwrap();
        linker.define("host", "mem", memory).unwrap();
        memory
            .write(&mut store, 0, &0b1011_i32.to_le_bytes())
            .unwrap();
        let instance = linker.instantiate(&mut store, &module).unwrap();
        let run = instance
            .get_typed_func::<i32, (i32, i32, i32, i32), _>(&mut store, "run")
            .unwrap();
        let result = run.call(&mut store, 0).unwrap();
        println!("{:?}", result);
        assert_eq!(result, (1, 0, 1, 1));
    }

    #[test]
    fn test_example_multiple() {
        let input = include_str!("example.txt");
        let program = input.parse().unwrap();
        let mut runner = Runner::build(&program);

        assert_eq!(runner.run(&[0b0000]), (0, 0, 0, 0));
        assert_eq!(runner.run(&[0b0001]), (0, 0, 0, 1));
        assert_eq!(runner.run(&[0b0010]), (0, 0, 1, 0));
        assert_eq!(runner.run(&[0b0011]), (0, 0, 1, 1));
        assert_eq!(runner.run(&[0b0100]), (0, 1, 0, 0));
        assert_eq!(runner.run(&[0b1000]), (1, 0, 0, 0));
        assert_eq!(runner.run(&[0b1111]), (1, 1, 1, 1));
    }

    #[test]
    fn test_interpreter_equivalence() {
        let input = include_str!("input.txt");
        let program: Program = input.parse().unwrap();

        let interpreter = crate::interpreter::Interpreter::build(&program);
        let mut wasm_runner = Runner::build(&program);

        let input = [1, 3, 5, 7, 9, 2, 4, 6, 8, 9, 9, 9, 9, 9];
        let interpreter_result = interpreter.run(&input);
        let wasm_runner_result = wasm_runner.run(&input);
        assert_eq!(interpreter_result, wasm_runner_result);
    }
}
