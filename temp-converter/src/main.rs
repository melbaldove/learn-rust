use std::io;

fn main() {
    // Get the temperature in fahrenheit
    // Compute the celsius equivalent
    // C = (F - 32) * 5/9
    let mut f = String::new();
    println!("Please input fahrenheit:");

    io::stdin().read_line(&mut f).expect("Failed to read line!");

    let f: f32 = f.trim().parse().expect("Please input a number!");

    let c = (f - 32.0) * (5.0 / 9.0);

    println!("Celsius: {c}");
}
