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

#[derive(Debug)]
enum Error {
    NotFileInput,
    FileNotFound,
    Serialize,
}

fn get_file_name() -> Result<String, Error> {
    let file = match std::env::args().nth(1) {
        Some(file) => file,
        None => return Err(Error::NotFileInput),
    };

    Ok(file)
}

fn open_file(file: &String) -> Result<String, Error> {
    let orders = match std::fs::read_to_string(file) {
        Ok(orders) => orders,
        Err(_e) => return Err(Error::FileNotFound),
    };

    Ok(orders)
}

fn serialize(orders: &str) -> Result<Orders, Error> {
    let orders = match serde_json::from_str::<Orders>(orders) {
        Ok(orders) => orders,
        Err(_) => return Err(Error::Serialize),
    };

    Ok(orders)
}

fn main() -> Result<(), Error> {
    let file = get_file_name()?;

    let orders = {
        let orders = open_file(&file)?;
        serialize(&orders)?
    };

    for idx in 0..orders.orders.len() {
        println!("Order: {:?}", orders.orders[idx]);
    }

    Ok(())
}
