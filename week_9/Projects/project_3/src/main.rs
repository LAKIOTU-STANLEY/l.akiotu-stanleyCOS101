use::std::io::Write;

fn main() {
    let commissioner = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];

    let ministry = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];

    let zone = vec!["South West","North East","South South","South West","South East"];

    let mut file = std::fs::File::create("EFCC_Convicted_Ministers_Merge.csv").expect("Create failed");

    let header = "Commissioner,Minintry,Zone\n";
    println!("{}",header);
    file.write_all(header.as_bytes()).expect("Write failed");

    for i in 0..commissioner.len()
{
    let line = commissioner[i].to_string() + "," + ministry[i] + "," + zone[i] + "\n";
    println!("{}",line);
    file.write_all(line.as_bytes()).expect("Write failed");
}
    println!("\nAll Commissioners have been saved to file");
}