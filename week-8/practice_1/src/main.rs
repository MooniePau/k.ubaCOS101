fn main() {
    
    //using Vec::new()
    let v:Vec<i64> = Vec::new();

    //print the size
    println!("\nThe length of Vec::new is {}", v.len());

    //using the macro function
    let v = vec!["grace", "Effiong", "Basil", "Karen", "Susan"];

    //printing the size
    println!("\nThe length of vec macro is {}", v.len());
}
