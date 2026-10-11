use std::io;

fn main() {
    println!("Select a shape:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Surface Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read");
    let choice: u32 = choice.trim().parse().unwrap_or(0);

    if choice == 1 {
        area_trapezium();
    } else if choice == 2 {
        area_rhombus();
    } else if choice == 3 {
        area_parallelogram();
    } else if choice == 4 {
        area_cube();
    } else if choice == 5 {
        volume_cylinder();
    } else {
        println!("Invalid option!");
    }
}

// 1. Trapezium, area: height / 2 * (base1 + base2)
fn area_trapezium() {
    println!("Enter height:");
    let mut h = String::new();
    io::stdin().read_line(&mut h).unwrap();
    let height: f64 = h.trim().parse().unwrap();

    println!("Enter base 1:");
    let mut b1 = String::new();
    io::stdin().read_line(&mut b1).unwrap();
    let base1: f64 = b1.trim().parse().unwrap();

    println!("Enter base 2:");
    let mut b2 = String::new();
    io::stdin().read_line(&mut b2).unwrap();
    let base2: f64 = b2.trim().parse().unwrap();

    let area = height / 2.0 * (base1 + base2);
    println!("Area of Trapezium = {}", area);
}

// 2. Rhombus, area: 1/2 * diagonal1 * diagonal2
fn area_rhombus() {
    println!("Enter diagonal 1:");
    let mut d1 = String::new();
    io::stdin().read_line(&mut d1).unwrap();
    let diagonal1: f64 = d1.trim().parse().unwrap();

    println!("Enter diagonal 2:");
    let mut d2 = String::new();
    io::stdin().read_line(&mut d2).unwrap();
    let diagonal2: f64 = d2.trim().parse().unwrap();

    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area of Rhombus = {}", area);
}

// 3. Parallelogram, area: base * altitude
fn area_parallelogram() {
    println!("Enter base:");
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let base: f64 = b.trim().parse().unwrap();

    println!("Enter altitude:");
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let altitude: f64 = a.trim().parse().unwrap();

    let area = base * altitude;
    println!("Area of Parallelogram = {}", area);
}

// 4. Cube, surface area: 6 * side * side
fn area_cube() {
    println!("Enter side:");
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let side: f64 = s.trim().parse().unwrap();

    let area = 6.0 * side * side;
    println!("Surface Area of Cube = {}", area);
}

// 5. Cylinder, volume: pi * radius * radius * height
fn volume_cylinder() {
    println!("Enter radius:");
    let mut r = String::new();
    io::stdin().read_line(&mut r).unwrap();
    let radius: f64 = r.trim().parse().unwrap();

    println!("Enter height:");
    let mut h = String::new();
    io::stdin().read_line(&mut h).unwrap();
    let height: f64 = h.trim().parse().unwrap();

    let pi = 3.14159;
    let volume = pi * radius * radius * height;
    println!("Volume of Cylinder = {}", volume);
}