use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plus_minus(n: i32, arr: &[i32]) {
    let mut pos = 0.0;
    let mut neg = 0.0;
    let mut z = 0.0;

    for a in arr.iter().copied() {
        if a > 0 {
            pos = pos + (1.0 / n as f32);
        } else if a < 0 {
            neg = neg + (1.0 / n as f32);
        } else {
            z = z + (1.0 / n as f32);
        }
    }

    println!("{}\n{}\n{}", pos, neg, z)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plus_minus(n, &arr);
}
