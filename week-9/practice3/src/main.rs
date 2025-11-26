use std::fs;
use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("data.txt").expect("Create Failed");
    file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("Write Failed");

    fs::remove_file("data.txt").expect("Could not remove file");
    println!("File is removed");
}
