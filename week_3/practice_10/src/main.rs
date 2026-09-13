fn main() {
    // addition
    let sum = 5550 + 7310;
    println!("The sum of 5550 and 7310 = {}", sum);

    // subtraction
    let difference: f64 = 95.5 - 4.3; // Changed u32 to f64 (floating point)
    println!("The difference of 95.5 and 4.3 = {}", difference); // Replaced () with {}

    // multiplication
    let product: f32 = 4.0 * 30.0; // Added decimal points for f32 floats
    println!("The multiple of 4 and 30 = {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The division of 56.7 and 32.2 = {}", quotient); // Fixed syntax in string formatting

    // remainder
    let remainder = 43 % 5;
    println!("The remainder of 43 and 5 = {}", remainder);
}