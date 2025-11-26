fn main() {
    
    let mut name = vec!["Mary", "Sum", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];
    println!("{}", name[2]);
    println!("Vector length: {}", name.len());

    let age  = vec![16,17,19,22,20,21,18,23];


    println!("\nAge allocation:\n");

    for i in 0..age.len(){

        println!("{} is {} years old\n",name[i], age[i]);
    }
}
