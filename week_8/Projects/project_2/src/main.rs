fn main() {
    println!("EY Nigeria Developer Experience Finder");

    // Vector of tuples holding (Candidate Name, Years of Experience)
    let candidates = vec![
        ("Chinedu", 4),
        ("Amina", 7),
        ("Tunde", 10),
        ("Folake", 5),
        ("David", 8),
    ];

    // Assume the first candidate has the highest experience initially
    let mut best_candidate = candidates[0].0;
    let mut max_exp = candidates[0].1;

    // Walk through the vector with a loop
    for candidate in &candidates {
        let (name, exp) = *candidate; // Destructure the tuple
        if exp > max_exp {
            max_exp = exp;
            best_candidate = name;
        }
    }

    // Print the final result
    println!("\nThe candidate with the highest experience is {} with {} years of programming experience.", best_candidate, max_exp);
}