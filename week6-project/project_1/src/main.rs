use std::io;

fn main() {
    loop {
        println!("Code\tFood Menu\t\t\tPrice (₦)");
        println!("P\tPounded yam/Edinkaiko soup\t3,200");
        println!("F\tFried Rice and Chicken\t\t3,000");
        println!("A\tAmala and Ewedu soup\t\t2,500");
        println!("E\tEba and Egusi soup\t\t2,000");
        println!("W\tWhite Rice & Stew\t\t2,500");

        let mut code = String::new();
        println!("Enter food code (P/F/A/E/W): ");
        io::stdin().read_line(&mut code).expect("Failed to read input");
        let foodcode = code.trim().to_uppercase();

        let mut qty_input = String::new();
        println!("Enter portion of food: ");
        io::stdin().read_line(&mut qty_input).expect("Failed to read input");
        let qty: i32 = qty_input.trim().parse().expect("Invalid number");

            let price: f64 = match foodcode.as_str() {
            "P" => 3_200.0,
            "F" => 3_000.0,
            "A" => 2_500.0,
            "E" => 2_000.0,
            "W" => 2_500.0,_ => {
                println!("Invalid Food code");
                continue;
            }
        };

        let total_cost = price * qty as f64;

        let final_amount = if qty > 3 {
            total_cost - (0.1 * total_cost)
        } else {
            total_cost
        };

        println!("--------------------------------");
        println!("Total cost of food: N{}", total_cost);
        if qty > 3 {
            println!("10% discount applied to purchase");
        }
        println!("Final amount to be payed: N{}", final_amount);
        println!("--------------------------------");

        let mut again = String::new();
        println!("Do you want to make another purchase?? (Y/N): ");
        io::stdin().read_line(&mut again).expect("Failed to read input");
        if again.trim().to_uppercase() != "Y" {
            break;
        }
    }

}
