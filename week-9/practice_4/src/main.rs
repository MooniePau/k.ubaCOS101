use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;

fn main() {
    File::create("data.txt").unwrap();

    let mut file = OpenOptions::new().append(true).open("data.txt").expect("Cannot open file");

    file.write_all("\nHello class".as_bytes()).expect("Write failed");
    file.write_all("\nThis is the appendage to the document."
        .as_bytes()).expect("Write failed");
    println!("File append success");
}