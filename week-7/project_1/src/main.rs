use std::io;
use std::f64::consts::PI;

fn take_input(prompt:& str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to read input");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid number.\n");
                continue;
            }
        }
    }
}

fn sqr(x:f64) -> f64 {
    x * x
}

fn calc_trapezium(height:f64, base1:f64, base2:f64) -> f64 {
    height / 2.0  * (base1 + base2)
}

fn calc_rhombus(diagonal1:f64, diagonal2:f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn calc_parallelogram(base:f64, altitude:f64) -> f64 {
    base * altitude 
}

fn calc_cube(length:f64) -> f64 {
    6.0 * sqr(length)
}

fn calc_cylinder(radius:f64, height:f64) -> f64 {
    PI * sqr(radius) * height
}

fn main() {

    'outer: loop {
        println!("Welcome to Area calculator! You enter the code of an area you want to calculate, and we'll calculate it for you.\n");

        println!("1. Area of trapezium.
        2. Area of calc_rhombus
        3. Area of Parallelogram.
        4. Area of Cube.
        5. Area of Cylinder.");

        let formula:f64 = take_input("Please enter a code below: ");

        match formula {
            1.0 => {
                let base1:f64 = take_input("Enter base 1: ");
                let base2:f64 = take_input("Enter base 2: ");
                let height:f64 = take_input("Enter height: ");

                let result:f64 = calc_trapezium(height, base1, base2);
                println!("Area of trapezium: {}\n", result);
            }

            2.0 => {
                let diagonal1:f64 = take_input("Enter diagonal 1: ");
                let diagonal2:f64 = take_input("Enter diagonal 2:");

                let result:f64 = calc_rhombus(diagonal1, diagonal2);
                println!("Area of rhombus: {}\n", result);
            }

            3.0 => {
                let base:f64 = take_input("Enter base: ");
                let altitude:f64 = take_input("Enter altitude");

                let result:f64 = calc_parallelogram(base, altitude);
                println!("Area of Parallelogram: {}", result);
            }

            4.0 => {
                let length:f64 = take_input("Enter length: ");

                let result:f64 = calc_cube(length);
                println!("Area of cube: {}", result);
            }

            5.0 => {
                let radius:f64 = take_input("Enter radius: ");
                let height:f64 = take_input("Enter height: ");

                let result:f64 = calc_cylinder(radius, height);
                println!("Area of cylinder: {}", result);
            }

            _ => {
                println!("You didn't enter a valid input. Please try again.\n");
                continue;
            }
        }

        loop {
            println!("Would you like to calculate another input? (y/n): ");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Unable to read input");

            let yes_no = input.trim();

            match yes_no {
                "y" | "Y" => {
                    continue 'outer;
                }

                "n" | "N" => {
                    println!("Alright! See you next time!");
                    break 'outer;
                }

                _ => {
                    println!("Enter either y / n: ");
                    continue 'outer;
                }
            }
        }
    }
}
