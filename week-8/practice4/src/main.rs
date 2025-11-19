fn main() {
    
    let name = vec!["Mary", "Sum", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];

    let age  = vec![16,17,19,22,20,21,18,23];

    println!("\nAge allocation:\n");

    for i in 0..age.len(){

        println!("{} is {} years old\n",name[i], age[i]);
    }
}
