// Fibonacci Recursive

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn fibonacci(s: i32) -> i32 {
    match s {
        0 => 0,
        1 => 1,
        _ => fibonacci(s - 1) + fibonacci(s - 2),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let result = fibonacci(s);

    writeln!(&mut fptr, "{}", result).ok();
}
