fn main() {
    
    //a name vector 
    let name = vec!["Mary", "Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];

    //an age vector
    let age = vec![16, 17, 19, 22, 20, 21, 18, 23];
    print!("\nAge allocation:\n");

    //loop to iterate elements in the vector
    for i in 0..age.len()
    {
        //iterating through i
        print!("{} is {} years old\n", name[i], age[i]);
    }
}
