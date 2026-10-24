use std::io::Write;


fn main() {
let header = ("Student Name","Matric_No","Course","Level");

let students = vec![
    ("Oluchi Mordi","ACC10211111","Accounting",300),
    ("Adams Aliyu","ECO10110101","Economics",100),
    ("Shania Bolade","CSC10328828","Computer Science",200),
    ("Adekunle Gold","EEE11020202","Electrical Engineering",200),
    ("Blanca Edemoh","MEE10202001","Mechanical Engineering",100),
    ];


let mut file = std::fs::File::create("PAU_SMIS_Records.csv").expect("Create failed");

let header = header.0.to_string() + "," + header.1 + "," + header.2 + "," + header.3 + "\n";
println!("{}",header);
file.write_all(header.as_bytes()).expect("Write failed");

for row in students.iter()
{


    let line = row.0.to_string() + "," + row.1 + "," + row.2 + "," + &row.3.to_string() + "\n";
    println!("{}",line);
    file.write_all(line.as_bytes()).expect("Write failed");
}    
 
    println!("\nData written to file");  
}   