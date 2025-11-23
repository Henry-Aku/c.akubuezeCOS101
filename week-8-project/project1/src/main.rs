use std::io;

// Function that asks for input 
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {

    // Vectors for job titles and APS levels
    let office_admin = vec![
        ("Intern", "APS 1-2"),
        ("Administrator", "APS 3-5"),
        ("Senior Administrator", "APS 5-8"),
        ("Office Manager", "EL1 8-10"),
        ("Director", "EL2 10-13"),
        ("CEO", "SES"),
    ];

    let academic = vec![
        ("Research Assistant", "APS 3-5"),
        ("PhD Candidate", "APS 5-8"),
        ("Post-Doc Researcher", "EL1 8-10"),
        ("Senior Lecturer", "EL2 10-13"),
        ("Dean", "SES"),
    ];

    let lawyer = vec![
        ("Paralegal", "APS 1-2"),
        ("Junior Associate", "APS 3-5"),
        ("Associate", "APS 5-8"),
        ("Senior Associate 1-2", "EL1 8-10"),
        ("Senior Associate 3-4", "EL2 10-13"),
        ("Partner", "SES"),
    ];

    let teacher = vec![
        ("Placement", "APS 1-2"),
        ("Classroom Teacher", "APS 3-5"),
        ("Snr Teacher", "APS 5-8"),
        ("Leading Teacher", "EL1 8-10"),
        ("Deputy Principal", "EL2 10-13"),
        ("Principal", "SES"),
    ];

    // Use function to get inputs
    let profession = get_input("Enter profession (Office / Academic / Lawyer / Teacher):").to_lowercase();
    let title = get_input("Enter job title:");

    // Select the right vector based on profession
    let selected_list = match profession.as_str() {
        "office" => office_admin,
        "academic" => academic,
        "lawyer" => lawyer,
        "teacher" => teacher,
        _ => {
            println!("Unknown profession.");
            return;
        }
    };

    // Search for the job title in the chosen vector
    for (job_title, aps_level) in selected_list {
        if job_title.eq_ignore_ascii_case(&title) {
            println!("Staff level: {}", aps_level);
            return;
        }
    }

    println!("Job title not found in this profession.");
}
