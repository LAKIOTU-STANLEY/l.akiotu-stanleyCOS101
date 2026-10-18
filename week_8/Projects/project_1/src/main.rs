use std::io;

fn main() {
    println!("Public Service APS Level Checker");

    // Input profession
    let mut profession = String::new();
    println!("Enter your profession (Office Admin, Academic, Lawyer, Teacher):");
    io::stdin().read_line(&mut profession).expect("Failed to read input");
    let profession = profession.trim().to_lowercase();

    // Input years of experience
    let mut experience_str = String::new();
    println!("Enter your years of experience (e.g., 6):");
    io::stdin().read_line(&mut experience_str).expect("Failed to read input");
    let exp: u32 = experience_str.trim().parse().expect("Please enter a valid number");

    // Determine APS level based on profession and experience
    let aps_level = match profession.as_str() {
        "office admin" => {
            if exp <= 2 { "APS 1-2: Intern" }
            else if exp <= 5 { "APS 3-5: Administrator" }
            else if exp <= 8 { "APS 5-8: Senior Administrator" }
            else if exp <= 10 { "EL1 8-10: Office Manager" }
            else if exp <= 13 { "EL2 10-13: Director" }
            else { "SES: CEO" }
        },
        "academic" => {
            if exp <= 2 { "APS 1-2: None (-)" }
            else if exp <= 5 { "APS 3-5: Research Assistant" }
            else if exp <= 8 { "APS 5-8: PhD Candidate" }
            else if exp <= 10 { "EL1 8-10: Post-Doc Researcher" }
            else if exp <= 13 { "EL2 10-13: Senior Lecturer" }
            else { "SES: Dean" }
        },
        "lawyer" => {
            if exp <= 2 { "APS 1-2: Paralegal" }
            else if exp <= 5 { "APS 3-5: Junior Associate" }
            else if exp <= 8 { "APS 5-8: Associate" }
            else if exp <= 10 { "EL1 8-10: Senior Associate 1-2" }
            else if exp <= 13 { "EL2 10-13: Senior Associate 3-4" }
            else { "SES: Partner" }
        },
        "teacher" => {
            if exp <= 2 { "APS 1-2: Placement" }
            else if exp <= 5 { "APS 3-5: Classroom Teacher" }
            else if exp <= 8 { "APS 5-8: Senior Teacher" }
            else if exp <= 10 { "EL1 8-10: Leading Teacher" }
            else if exp <= 13 { "EL2 10-13: Deputy Principal" }
            else { "SES: Principal" }
        },
        _ => "Unknown profession entered.",
    };

    println!("\nResult: Your designated level is -> {}", aps_level);
}