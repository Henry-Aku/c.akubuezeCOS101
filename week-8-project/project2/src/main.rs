use std::io;

fn main() {
    let mut applicants: Vec<(String, u32)> = Vec::new();

    println!("How many applicants were interviewed?");
    let total_applicants = read_number();

    // Collect applicant data
    for index in 1..=total_applicants {
        println!("\nEnter the name of applicant {}:", index);
        let applicant_name = read_string();

        println!("Enter years of programming experience:");
        let years_experience = read_number();

        // Store as tuple: (name, experience)
        applicants.push((applicant_name, years_experience));
    }

    // Assume first applicant has the highest experience
    let mut top_applicant = &applicants[0];

    // Find applicant with most experience
    for applicant in &applicants {
        if applicant.1 > top_applicant.1 {
            top_applicant = applicant;
        }
    }

    println!("\nApplicant with the highest experience:");
    println!("Name: {}", top_applicant.0);
    println!("Experience: {} years", top_applicant.1);
}


fn read_string() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read input");
    buffer.trim().to_string()
}

fn read_number() -> u32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read number");
    buffer.trim().parse().expect("Please enter a valid number")
}
