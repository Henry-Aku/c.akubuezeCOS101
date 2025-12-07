struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn cost_for(&self, qty: u32) -> u32 {
        self.price * qty
    }
}

fn main() {
    let hp = Laptop { brand: "HP".to_string(), price: 650_000 };
    let ibm = Laptop { brand: "IBM".to_string(), price: 755_000 };
    let toshiba = Laptop { brand: "Toshiba".to_string(), price: 550_000 };
    let dell = Laptop { brand: "Dell".to_string(), price: 850_000 };

    // Customer buys 3 of each brand
    let qty = 3;

    let total_cost =
          hp.cost_for(qty)
        + ibm.cost_for(qty)
        + toshiba.cost_for(qty)
        + dell.cost_for(qty);

    println!("Total cost for purchasing 3 from each brand is: {}", total_cost);
}
