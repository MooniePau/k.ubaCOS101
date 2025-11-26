use std::fs::File;
use std::fs;

fn main() {
    File::create("data.txt").unwrap();
    fs::remove_file("data.txt").expect("Could not remove file");

    println!("File is removed");
}