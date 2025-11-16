use std::io;

fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().parse().expect("Enter a valid number")
}



fn area_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn area_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn area_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_cube(side: f64) -> f64 {
    6.0 * side.powi(2)
}

fn volume_cylinder(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius.powi(2) * height
}

fn main() {
    println!("Select a formula to calculate:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");
    println!("Enter your choice: ");

    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input).expect("Failed");
    let choice: u32 = choice_input.trim().parse().expect("Enter a valid number");

    match choice {
        1 => {
            let height = get_input("Enter height: ");
            let base1 = get_input("Enter base1: ");
            let base2 = get_input("Enter base2: ");
            let result = area_trapezium(height, base1, base2);
            println!("Area of Trapezium = {}", result);
        }

        2 => {
            let d1 = get_input("Enter diagonal 1: ");
            let d2 = get_input("Enter diagonal 2: ");
            let result = area_rhombus(d1, d2);
            println!("Area of Rhombus = {}", result);
        }

        3 => {
            let base = get_input("Enter base: ");
            let alt = get_input("Enter altitude: ");
            let result = area_parallelogram(base, alt);
            println!("Area of Parallelogram = {}", result);
        }

        4 => {
            let side = get_input("Enter length of side: ");
            let result = area_cube(side);
            println!("Area of Cube = {}", result);
        }

        5 => {
            let radius = get_input("Enter radius: ");
            let height = get_input("Enter height: ");
            let result = volume_cylinder(radius, height);
            println!("Volume of Cylinder = {}", result);
        }

        _ => println!("Invalid choice!"),
    }
}

