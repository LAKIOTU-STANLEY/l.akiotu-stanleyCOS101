struct Laptop {
    brand: String,
    quantity: u32,
    price: u32
}

impl Laptop {
    fn total_cost(&self)->u32 {
        self.quantity * self.price
    }
}

fn main(){
    let hp = Laptop{
        brand: String::from("HP"),
        quantity: 3,
        price: 650_000
    };
    let ibm = Laptop{
        brand: String::from("IBM"),
        quantity: 3,
        price: 755_000
    };
    let toshiba = Laptop{
        brand : String::from("Toshiba"),
        quantity: 3,
        price: 550_000
    };
    let dell = Laptop{
        brand: String::from("Dell"),
        quantity: 3,
        price: 850_000
    };

    println!("{} cost: N{}", hp.brand, hp.total_cost());
    println!("{} cost: N{}", ibm.brand, ibm.total_cost());
    println!("{} cost: N{}", toshiba.brand, toshiba.total_cost());
    println!("{} cost: N{}", dell.brand, dell.total_cost());

    let total = hp.total_cost() + ibm.total_cost() + toshiba.total_cost() + dell.total_cost();
    println!("Total for 3 laptops from each brand: N{}", total);
}