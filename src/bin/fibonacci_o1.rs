// Big Fibonacci

use std::{
    env,
    fs::File,
    io::{self, BufRead, Write},
};

fn fibonacci(n: u128) -> u128 {
    if n == 1 || n == 2 {
        return 1;
    }

    let mut s = vec![1, 1];
    let mut sum: u128 = 0;

    for _ in 2..n {
        sum = {
            let a: u128 = s[0].clone();
            let b: u128 = s[1].clone();
            let Some(ss) = a.checked_add(b) else {
                panic!("Overflow!!!");
            };
            ss
        };
        s[0] = s[1];
        s[1] = sum;
    }

    sum
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .parse::<u128>()
        .unwrap();

    let result = fibonacci(s);

    writeln!(&mut fptr, "{}", result).ok();
}
