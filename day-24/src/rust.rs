pub struct Runner;

impl Runner {
    pub fn run(&self, input: &[i32]) -> (i32, i32, i32, i32) {
        run_input(input)
    }
}

struct DataState<'a> {
    inputs: &'a [i32],
    w: i32,
    x: i32,
    y: i32,
    z: i32,
    input_index: usize,
}

macro_rules! as_rust {
    ($ident:ident { $($tt:tt)* }) => {
        pub fn $ident(inputs: &[i32]) -> (i32, i32, i32, i32) {
            let mut state = DataState {
                inputs,
                w: 0,
                x: 0,
                y: 0,
                z: 0,
                input_index: 0,
            };

            as_rust!(@build state; $($tt)*);

            (state.w, state.x, state.y, state.z)
        }
    };
    (@build $state:ident; inp $var:ident $($tt:tt)*) => {
        $state.$var = $state.inputs[$state.input_index];
        $state.input_index += 1;
        as_rust!(@build $state; $($tt)*);
    };
    (@build $state:ident; add $var1:ident $var2:ident $($tt:tt)*) => {
        $state.$var1 += $state.$var2;
        as_rust!(@build $state; $($tt)*);
    };
    (@build $state:ident; add $var1:ident $n:literal $($tt:tt)*) => {
        $state.$var1 += $n;
        as_rust!(@build $state; $($tt)*);
    };
    (@build $state:ident; mul $var1:ident $var2:ident $($tt:tt)*) => {
        $state.$var1 *= $state.$var2;
        as_rust!(@build $state; $($tt)*);
    };
    (@build $state:ident; mul $var1:ident $n:literal $($tt:tt)*) => {
        $state.$var1 *= $n;
        as_rust!(@build $state; $($tt)*);
    };
    (@build $state:ident; div $var1:ident $var2:ident $($tt:tt)*) => {
        $state.$var1 /= $state.$var2;
        as_rust!(@build $state; $($tt)*);
    };
    (@build $state:ident; div $var1:ident $n:literal $($tt:tt)*) => {
        $state.$var1 /= $n;
        as_rust!(@build $state; $($tt)*);
    };
    (@build $state:ident; rem $var1:ident $var2:ident $($tt:tt)*) => {
        $state.$var1 %= $state.$var2;
        as_rust!(@build $state; $($tt)*);
    };
    (@build $state:ident; rem $var1:ident $n:literal $($tt:tt)*) => {
        $state.$var1 %= $n;
        as_rust!(@build $state; $($tt)*);
    };
    (@build $state:ident; mod $var1:ident $var2:ident $($tt:tt)*) => {
        $state.$var1 %= $state.$var2;
        as_rust!(@build $state; $($tt)*);
    };
    (@build $state:ident; mod $var1:ident $n:literal $($tt:tt)*) => {
        $state.$var1 %= $n;
        as_rust!(@build $state; $($tt)*);
    };
    (@build $state:ident; eql $var1:ident $var2:ident $($tt:tt)*) => {
        $state.$var1 = if $state.$var1 == $state.$var2 { 1 } else { 0 };
        as_rust!(@build $state; $($tt)*);
    };
    (@build $state:ident; eql $var1:ident $n:literal $($tt:tt)*) => {
        $state.$var1 = if $state.$var1 == $n { 1 } else { 0 };
        as_rust!(@build $state; $($tt)*);
    };
    (@build $state:ident; ) => {};
}

pub(crate) use as_rust;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        as_rust!(run_basic {
            inp w
            inp x
            inp y
        });

        assert_eq!(run_basic(&[1, 2, 3]), (1, 2, 3, 0));
    }

    #[test]
    fn test_example() {
        as_rust!(run_example {
            inp w
            add z w
            mod z 2
            div w 2
            add y w
            mod y 2
            div w 2
            add x w
            mod x 2
            div w 2
            mod w 2
        });

        assert_eq!(run_example(&[0b0000]), (0, 0, 0, 0));
        assert_eq!(run_example(&[0b0001]), (0, 0, 0, 1));
        assert_eq!(run_example(&[0b0010]), (0, 0, 1, 0));
        assert_eq!(run_example(&[0b0011]), (0, 0, 1, 1));
        assert_eq!(run_example(&[0b0100]), (0, 1, 0, 0));
        assert_eq!(run_example(&[0b1000]), (1, 0, 0, 0));
        assert_eq!(run_example(&[0b1111]), (1, 1, 1, 1));
    }

    #[test]
    fn test_interpreter_equivalence() {
        let input = include_str!("input.txt");
        let program: crate::program::Program = input.parse().unwrap();

        let interpreter = crate::interpreter::Interpreter::build(&program);

        let input = [1, 3, 5, 7, 9, 2, 4, 6, 8, 9, 9, 9, 9, 9];
        let interpreter_result = interpreter.run(&input);
        let wasm_runner_result = run_input(&input);
        assert_eq!(interpreter_result, wasm_runner_result);
    }
}

as_rust!(run_input {
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 11
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 1
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 10
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 10
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 13
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 2
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -10
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 5
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 11
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 6
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 11
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 0
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 12
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 16
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -11
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 12
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -7
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 15
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 1
    add x 13
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 7
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -13
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 6
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x 0
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 5
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x -11
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 6
    mul y x
    add z y
    inp w
    mul x 0
    add x z
    mod x 26
    div z 26
    add x 0
    eql x w
    eql x 0
    mul y 0
    add y 25
    mul y x
    add y 1
    mul z y
    mul y 0
    add y w
    add y 15
    mul y x
    add z y
});
