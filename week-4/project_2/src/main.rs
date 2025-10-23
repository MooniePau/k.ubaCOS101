use std::io;

fn main() {
    println!("Age? ");

    let mut input1 = String::new();

    io::stdin()
    .read_line(&mut input1)
    .expect("Invalid String!");

    let age:f64 = input1.trim().parse().expect("Invalid number!");

    println!("Years of experience? ");

    let mut input2 = String::new();

    io::stdin()
    .read_line(&mut input2)
    .expect("Invalid String!");

    let years_of_exp:f64 = input2.trim().parse().expect("invalid number!");

    if years_of_exp >= 2.0 && age >= 40.0 {
        println!("Employee Incentive: N 1,560,000.");
    }
    else if years_of_exp >= 2.0 && age >= 30.0 && age < 40.0 {
        println!("Employee Incentive: N 1,480,000.");
    }
    else if years_of_exp >= 2.0 && age < 28.0 {
        println!("Employee Incentive: N 1,300,000.");
    }
    else {
        println!("Employee Incentive: N 100,000");
    }

}