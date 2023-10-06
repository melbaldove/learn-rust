use std::io;

fn main() {
    let mut n = String::new();
    println!("Input the nth fib number to compute:");
    io::stdin().read_line(&mut n).expect("Failed to readline");

    let n: u32 = n.trim().parse().expect("Please input a number!");

    let mut a = 0;
    let mut b = 1;

    for _ in 0..n - 1 {
        let new_a = b;
        let new_b = a + b;
        a = new_a;
        b = new_b;
    }

    println!("The {n}th fibonacci number is {a}");
}
