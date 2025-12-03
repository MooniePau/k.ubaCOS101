fn main() {
    //list of numbers
    let x = vec![100, 200, 300];
    borrow_vector(&x); //passing by reference

    println!("Printing the value from main() x[] = {}", x[0]);
    println!("*****************************");
}

fn borrow_vector(z:&Vec<i32>) {

    println!("************************************");
    println!("Inside the borrow_vector function {:?} \n", z);
    println!("-----------------------------------");
}
