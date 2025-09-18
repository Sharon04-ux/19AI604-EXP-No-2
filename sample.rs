// Sample Rust Program for Experiment 2
// Try building something similar
fn main() {
    // Printing a welcome message
    println!("Welcome to Rust Programming with Cargo!");

    // Variables
    let name = "Student";
    let reg_no = 12345;

    println!("Name: {}", name);
    println!("Register Number: {}", reg_no);

    // Simple calculation
    let a = 10;
    let b = 20;
    let sum = a + b;
    println!("The sum of {} and {} is {}", a, b, sum);

    // Conditional statement
    if sum > 15 {
        println!("The sum is greater than 15");
    } else {
        println!("The sum is 15 or less");
    }

    // Loop example
    println!("Counting from 1 to 5:");
    for i in 1..=5 {
        println!("{}", i);
    }
}
