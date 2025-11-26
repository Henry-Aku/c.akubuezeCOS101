use std::io::Write;
use std::io::Read;

fn main() {

    let mut this_file = std::fs::File::create("Welcome_message.txt").expect("Create Failed");
    this_file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("Write Failed");

    let mut file = std::fs::File::open("Welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
