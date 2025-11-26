use std::io::{Read, Write};
use std::fs::File;

fn main () {
    {   let mut file = File::create("welcome_message.txt").unwrap();
        file.write_all("Hola, soy Dora!".as_bytes()).unwrap();
    }


    let mut file = File::open("welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{}", contents);

}
