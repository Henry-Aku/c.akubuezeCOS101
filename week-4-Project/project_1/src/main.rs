use std::io;

fn main() {
    let mut a_input = String::new();
    let mut b_input = String::new();
    let mut c_input = String::new();

    // Input values for a, b, and c
    println!("Enter the value of a: ");
    io::stdin().read_line(&mut a_input).expect("Failed to read input");
    let a: f64 = a_input.trim().parse().expect("Please enter a valid number");

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut b_input).expect("Failed to read input");
    let b: f64 = b_input.trim().parse().expect("Please enter a valid number");

    println!("Enter the value of c: ");
    io::stdin().read_line(&mut c_input).expect("Failed to read input");
    let c: f64 = c_input.trim().parse().expect("Please enter a valid number");

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("There are two distinct real roots:");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("There is exactly one real root:");
        println!("Root = {}", root);
    } else {
        println!("There are no real roots since the discriminant is negative.");
    }
}
