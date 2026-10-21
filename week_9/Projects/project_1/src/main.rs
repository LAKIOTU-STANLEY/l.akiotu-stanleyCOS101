use std::io::Write;

fn main() {
    let header = "Lager, Stout, Non-Alcoholic\n";
    let row_1 = "33 Export, Legend, Maltina\n";
    let row_2 = "Desperados, Turbo King, Amstel Malta\n";
    let row_3 = "Goldberg, Wiliams, Malta Gold\n";
    let row_4 = "Gulder, , Fayrouz\n";
    let row_5 = "Heineken, , \n";
    let row_6 = "Star, , \n";

    let mut file = std::fs::File::create("nigerian_breweries.csv").expect("create failed");
    file.write_all("Nigerian Breweries\n".as_bytes()).expect("Write failed");
    file.write_all(header.as_bytes()).expect("Write failed");
    file.write_all(row_1.as_bytes()).expect("Write failed");
    file.write_all(row_2.as_bytes()).expect("Write failed");
    file.write_all(row_3.as_bytes()).expect("Write failed");
    file.write_all(row_4.as_bytes()).expect("Write failed");
    file.write_all(row_5.as_bytes()).expect("Write failed");
    file.write_all(row_6.as_bytes()).expect("Write failed");
    println!("\nNigerian Breweries Drink Categories");
}