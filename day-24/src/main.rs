#![recursion_limit = "1024"]

//! No intention of this one actually finishing. I was just curious how long different approaches
//! might take.
//!
//! So I took three approaches:
//!
//! 1. "interpreter": an interpreter; the default approach to solving a problem like this
//! 2. "wasm": transpile the program to "wast", use "wasmtime" to compile that to "wasm", and then
//!    run it using wasmtime.
//! 3. "rust" (aka "macro"): write a macro that builds a rust function from the input direction.
//!    Presumably rust can make this very fast.
//!
//! With those three approaches, I get this output on my MacBook Pro (16-inch, 2019)
//!
//! bryan ~/personal/advent-of-code-2021 $ cargo run -q --release -p day-24 -- --all
//! interpreter  100 million serial numbers in   74s, expected duration 196d  3h 48m 48s
//! wasm         100 million serial numbers in   15s, expected duration  42d  1h 11m 55s
//! macro        100 million serial numbers in    5s, expected duration  14d 18h 47m 21s

use std::time::{Duration, Instant};

use serial_number_iterator::SerialNumberIterator;

mod interpreter;
mod program;
mod rust;
mod serial_number_iterator;
mod wasm;

fn main() {
    let input = std::env::args().nth(1);
    let input = input.as_deref();
    let program = include_str!("input.txt");
    let program = program.parse().unwrap();

    let mut runner = match input {
        Some("--interpreter") => Runner::Interpreter(interpreter::Interpreter::build(&program)),
        Some("--wasm") => Runner::Wasm(wasm::Runner::build(&program)),
        Some("--rust") => Runner::Rust(rust::Runner),
        Some("--all") => Runner::All(
            interpreter::Interpreter::build(&program),
            wasm::Runner::build(&program),
            rust::Runner,
        ),
        Some(a) => panic!(
            "Expected '--interpreter', '--wasm', '--rust', '--all'; found '{}'",
            a
        ),
        None => panic!("Expected '--interpreter', '--wasm', '--rust', '--all'; found nothing"),
    };

    runner.run();
}

pub enum Runner {
    Interpreter(interpreter::Interpreter),
    Wasm(wasm::Runner),
    Rust(rust::Runner),
    All(interpreter::Interpreter, wasm::Runner, rust::Runner),
}

impl Runner {
    pub fn run(&mut self) {
        match self {
            Self::Interpreter(interpreter) => {
                let iterator: SerialNumberIterator<14> = Default::default();
                let mut count = 0;
                let mut reset = 0;
                let start = Instant::now();
                for serial in iterator {
                    interpreter.run(&serial);
                    count += 1;
                    reset += 1;
                    if reset == RESET {
                        reset = 0;
                        output("interpreter", count, Instant::now() - start);
                    }
                }
            }
            Self::Wasm(runner) => {
                let iterator: SerialNumberIterator<14> = Default::default();
                let mut count = 0;
                let mut reset = 0;
                let start = Instant::now();
                for serial in iterator {
                    runner.run(&serial);
                    count += 1;
                    reset += 1;
                    if reset == RESET {
                        reset = 0;
                        output("wasm", count, Instant::now() - start);
                    }
                }
            }
            Self::Rust(runner) => {
                let iterator: SerialNumberIterator<14> = Default::default();
                let mut count = 0;
                let mut reset = 0;
                let start = Instant::now();
                for serial in iterator {
                    runner.run(&serial);
                    count += 1;
                    reset += 1;
                    if reset == RESET {
                        reset = 0;
                        output("macro", count, Instant::now() - start);
                    }
                }
            }
            Self::All(interpreter, wasm, rust) => {
                const MAX: usize = 100_000_000;

                let mut count = 0;
                let iterator: SerialNumberIterator<14> = Default::default();
                let start = Instant::now();
                for serial in iterator {
                    interpreter.run(&serial);
                    count += 1;
                    if count == MAX {
                        output("interpreter", count, Instant::now() - start);
                        break;
                    }
                }

                let mut count = 0;
                let iterator: SerialNumberIterator<14> = Default::default();
                let start = Instant::now();
                for serial in iterator {
                    wasm.run(&serial);
                    count += 1;
                    if count == MAX {
                        output("wasm", count, Instant::now() - start);
                        break;
                    }
                }

                let mut count = 0;
                let iterator: SerialNumberIterator<14> = Default::default();
                let start = Instant::now();
                for serial in iterator {
                    rust.run(&serial);
                    count += 1;
                    if count == MAX {
                        output("macro", count, Instant::now() - start);
                        break;
                    }
                }
            }
        }
    }
}

const RESET: usize = 1_000_000;
const TOTAL: usize = 9 * 9 * 9 * 9 * 9 * 9 * 9 * 9 * 9 * 9 * 9 * 9 * 9 * 9;

pub fn output(name: &str, count: usize, duration: Duration) {
    let total_micros = (duration.as_micros() as f64 * TOTAL as f64 / count as f64).floor() as u64;
    let total_duration = Duration::from_micros(total_micros);
    println!(
        "{:11}  {:3} million serial numbers in {:4}s, expected duration {}",
        name,
        count / 1_000_000,
        duration.as_secs(),
        format_expected_duration(total_duration),
    )
}

pub fn format_expected_duration(duration: Duration) -> String {
    let mut v = duration.as_secs();
    let secs = v % 60;
    v /= 60;
    let minutes = v % 60;
    v /= 60;
    let hours = v % 24;
    v /= 24;
    let days = v;
    format!("{:3}d {:2}h {:2}m {:2}s", days, hours, minutes, secs)
}
