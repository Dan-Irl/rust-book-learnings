use std::io;

fn main() {
    
    println!("What Fibonacci number do you want to calculate?");

    let mut n: String = String::new();
    
    io::stdin().
        read_line(&mut n).
        expect("Failed to read line");

    let n: u64 = n.trim().parse().expect("Please enter a valid number");

    println!("{}", fibonacci(n));
}


// Non recusive fibbonacci
fn fibonacci (n:u64) -> u64 {
    let mut x1: u64 = 0;
    let mut x2: u64 = 1;
    let mut result: u64 = 0;
    for _i in 1..n {
        result = x1 + x2;
        x1 = x2;
        x2 = result;
    }
    return result;
}