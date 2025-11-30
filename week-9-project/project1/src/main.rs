use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    // Each category is a vector of drink names (strings)
    let lager = vec![
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];

    let stout = vec![
        "Legend",
        "Turbo King",
        "Williams",
    ];

    let non_alcoholic = vec![
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz",
    ];

    // Create a file to write into
    let mut file = File::create("nb_drinks.txt")?;

    // Write each category into the file
    writeln!(file, "LAGER DRINKS:")?;
    for item in &lager {
        writeln!(file, "- {}", item)?;
    }

    writeln!(file, "\nSTOUT DRINKS:")?;
    for item in &stout {
        writeln!(file, "- {}", item)?;
    }

    writeln!(file, "\nNON-ALCOHOLIC DRINKS:")?;
    for item in &non_alcoholic {
        writeln!(file, "- {}", item)?;
    }

    println!("File created successfully: nb_drinks.txt");

    Ok(())
}
