use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Orders {
    pub all: Vec<Order>,
}

#[derive(Deserialize, Debug)]
pub struct Order {
    pub coffee: u32,
    pub water: u32,
    pub cocoa: u32,
    pub foam: u32,
}

impl Order {
    pub fn new(
        coffee_quantity: u32,
        water_quantity: u32,
        cocoa_quantity: u32,
        foam_quantity: u32,
    ) -> Order {
        Order {
            coffee: coffee_quantity,
            water: water_quantity,
            cocoa: cocoa_quantity,
            foam: foam_quantity,
        }
    }
}
