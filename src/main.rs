mod executor;

use crate::executor::{concurrent, wait_until_next_poll};

fn main() {
    concurrent(print_fibonacci(5), print_squares(5));
}

async fn print_fibonacci(amount: u64) {
    assert!(amount >= 2);
    println!("fib(0) = 0");
    wait_until_next_poll().await;
    println!("fib(1) = 1");

    let mut state = [0u64, 1];
    for i in 2..amount {
        wait_until_next_poll().await;

        let next = state[0] + state[1];
        state[0] = state[1];
        state[1] = next;
        println!("fib({i}) = {next}");
    }
}

async fn print_squares(amount: u64) {
    for i in 0..amount {
        println!("sqr({i}) = {}", i * i);
        wait_until_next_poll().await;
    }
}
