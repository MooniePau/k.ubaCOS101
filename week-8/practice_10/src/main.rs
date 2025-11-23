fn main() {
    let b:(i32, bool, f64) = (30, true, 4.9);
    print(b);
}

fn print(x:(i32, bool, f64)) {
    println!("Inside print method");

    //assign a tuplet odistinct variables
    let (age, is_male, cgpa) = x;
    println!("Age is {}, is_male is {}, cgpa is {}", age, is_male, cgpa);
}
