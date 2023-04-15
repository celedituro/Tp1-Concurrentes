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
