
use std::io;

fn main() {
    let mut name = String::new();
    let mut input2 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Not a valid string");

    println!("Enter your age");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if age >= 18 {
        println!("Welcome to the party {}!", name);
    }
    else {
        println!("Oops, you aren't of age to enter the party, {}", name);
    }
}