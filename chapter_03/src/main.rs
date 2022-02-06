use std::io;

const N_DEFAULT: usize = 100;

fn main() {
    // calculate nth Fibonacci
    println!(
        "Please input n (if no input is given, default to {})",
        N_DEFAULT
    );

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    // if input is empty, use default value, and calculate with memo
    if n.len() == 1 {
        let mut memo: [u128; N_DEFAULT + 1] = [0; N_DEFAULT + 1];
        println!(
            "result (default value): {}",
            fibonacci_with_memo(N_DEFAULT, &mut memo)
        )
    // else, calculate without memo
    } else {
        let n: u32 = n.trim().parse().expect("Failed to parse input");
        println!("result: {}", fibonacci_without_memo(n))
    }
}

fn fibonacci_without_memo(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_without_memo(n - 1) + fibonacci_without_memo(n - 2),
    }
}

fn fibonacci_with_memo(n: usize, memo: &mut [u128]) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            if memo[n] > 0 {
                memo[n]
            } else {
                let val = fibonacci_with_memo(n - 1, memo) + fibonacci_with_memo(n - 2, memo);
                memo[n] = val;
                val
            }
        }
    }
}
