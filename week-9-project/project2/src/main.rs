use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    // Vector of tuples: (Name, Matric Number, Department, Level)
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10382828", "Computer", 200),
        ("Adekunle Gold", "EEE10202002", "Electrical", 200),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    // Display header
    println!("PAU SMIS");
    println!("{:<20} {:<15} {:<15} {:<5}", 
             "Student Name", "Matric Number", "Department", "Level");
    println!("-----------------------------------------------------------------");

    // Display student details
    for student in &students {
        println!("{:<20} {:<15} {:<15} {:<5}", 
            student.0, student.1, student.2, student.3);
    }

    // Create the output file
    let mut file = File::create("student_records.txt")?;

    // Write header into the file
    writeln!(file, "PAU SMIS")?;
    writeln!(file, "{:<20} {:<15} {:<15} {:<5}", 
             "Student Name", "Matric Number", "Department", "Level")?;
    writeln!(file, "-----------------------------------------------------------------")?;

    // Write each student into the file
    for student in &students {
        writeln!(file, "{:<20} {:<15} {:<15} {:<5}", 
                 student.0, student.1, student.2, student.3)?;
    }

    println!("\nStudent details saved to student_records.txt");

    Ok(())
}
