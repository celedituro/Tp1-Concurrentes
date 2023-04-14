use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Order {
    coffee: u32,
    water: u32,
    cocoa: u32,
    foam: u32,
}

#[derive(Deserialize, Serialize, Debug)]
struct Orders {
    orders: Vec<Order>,
}

fn main() -> Result<(), std::io::Error> {
    let file = std::env::args().nth(1).unwrap();

    let orders = {
        let orders = std::fs::read_to_string(&file)?;

        serde_json::from_str::<Orders>(&orders).unwrap()
    };

    for index in 0..orders.orders.len() {
        println!("Order: {:?}", orders.orders[index]);
    }

    Ok(())
}
