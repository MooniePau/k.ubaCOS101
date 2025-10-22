//solving a quadratic formula using user input
use std::io;

fn main () {

    //prompt user for value a
    println!("What is value a? ");
    let mut input1 = String::new();
    io::stdin()
    .read_line(&mut input1)
    .expect("Invalid String");

    //convert to float
    let a:f64 = input1.trim().parse().expect("Invalid number");

    //prompt user for value b
    println!("What is value b? ");
    let mut input2 = String::new();
    io::stdin()
    .read_line(&mut input2)
    .expect("Invalid String");

    //convert to float
    let b:f64 = input2.trim().parse().expect("Invalid number");

    //prompt user for value c
    println!("What is value c? ");
    let mut input3 = String::new();
    io::stdin()
    .read_line(&mut input3)
    .expect("Invalid string");

    //convert to float 
    let c:f64 = input3.trim().parse().expect("Invalid number");

    let discriminant = b.powi(2) - (4.0 * a * c);

    if discriminant > 0.0 {
        let root1:f64 = (-b + discriminant.sqrt()) / 2.0;
        let root2:f64 = (-b - discriminant.sqrt()) / 2.0;

        println!("Roots (x): {}, {}.", root1, root2);
    }
    else if discriminant == 0.0 {
        let root3:f64 = -b / 2.0;
        println!("Single root: {}", root3);
    }
    else {
        println!("No real roots");
    }


}