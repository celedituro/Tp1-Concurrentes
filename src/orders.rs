use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Orders {
    pub all: Vec<Order>,
}

#[derive(Deserialize, Debug)]
pub struct Order {
    _coffee: Option<u32>,
    _water: Option<u32>,
    _cocoa: Option<u32>,
    _foam: Option<u32>,
}
