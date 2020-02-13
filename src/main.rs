use std::io;

fn main() {
    println!("Please type the nth number you'd like in the Fibonacci sequence.");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading line");

    let number: f64 = input.trim().parse().expect("Please provide a number");

    let result = fib(number).round();

    println!("The nth number is {}", result);
}

fn fib(number: f64) -> f64 {
    // https://www.geeksforgeeks.org/program-for-nth-fibonacci-number/

    // this seems silly
    // why can't I say `5.0.sqrt()`?
    // Better way? What am I missing?
    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;

    phi.powf(number) / 5.0_f64.sqrt()
}
