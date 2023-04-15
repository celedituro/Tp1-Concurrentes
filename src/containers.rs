use std::collections::HashMap;

use crate::orders::Order;

const MAX_COFFEE: u32 = 100;
const MAX_WATER: u32 = 10;
const MAX_COCOA: u32 = 5;
const MAX_FOAM: u32 = 50;

#[derive(Debug)]
pub struct Containers {
    pub all: HashMap<String, u32>,
}

impl Containers {
    pub fn new() -> Containers {
        let mut containers = HashMap::<String, u32>::new();
        containers.insert(String::from("coffee"), MAX_COFFEE);
        containers.insert(String::from("water"), MAX_WATER);
        containers.insert(String::from("cocoa"), MAX_COCOA);
        containers.insert(String::from("foam"), MAX_FOAM);

        Containers { all: containers }
    }

    pub fn get_drink(&mut self, order: &Order) {
        self.get_ingredient(&"coffee".to_string(), order.coffee);
        self.get_ingredient(&"water".to_string(), order.water);
        self.get_ingredient(&"cocoa".to_string(), order.cocoa);
        self.get_ingredient(&"foam".to_string(), order.foam);
    }

    pub fn get_ingredient(&mut self, ingredient: &String, value: u32) {
        let update_value = self.all[ingredient] - value;
        self.all.insert(String::from(ingredient), update_value);
    }
}

impl Default for Containers {
    fn default() -> Self {
        Self::new()
    }
}
