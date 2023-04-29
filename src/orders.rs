use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Order {
    pub coffee: u32,
    pub water: u32,
    pub cocoa: u32,
    pub foam: u32,
}

impl Order {
    /// Creates a new [`Order`].
    pub fn new(coffee_value: u32, water_value: u32, cocoa_value: u32, foam_value: u32) -> Order {
        Order {
            coffee: coffee_value,
            water: water_value,
            cocoa: cocoa_value,
            foam: foam_value,
        }
    }
}
