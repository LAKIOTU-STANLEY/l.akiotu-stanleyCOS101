use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Is the employee experienced? (true/false): ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let is_experienced: bool = input1.trim().parse().expect("Please enter true or false");

    println!("Enter age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age: i32 = input2.trim().parse().expect("Not a valid number");

    let incentive: i32;

    if is_experienced {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age <= 39 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            incentive = 0;
        }
    } else {
        incentive = 100_000;
    }

    if incentive > 0 {
        println!("Annual Incentive: N{}", incentive);
    } else {
        println!("No incentive criteria met for this age bracket.");
    }
}