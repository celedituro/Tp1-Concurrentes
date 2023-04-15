use std::collections::HashMap;

use crate::orders::Order;

const MAX_COFFEE: u32 = 5;
const MAX_WATER: u32 = 10;
const MAX_COCOA: u32 = 2;
const MAX_FOAM: u32 = 2;

const COFFEE: &str = "coffee";
const WATER: &str = "water";
const COCOA: &str = "cocoa";
const FOAM: &str = "foam";

#[derive(Debug)]
pub struct Containers {
    pub all: HashMap<String, u32>,
}

impl Containers {
    // Creates a container with all its containers of ingredients with theirs given availability
    pub fn new() -> Containers {
        let mut containers = HashMap::<String, u32>::new();
        containers.insert(COFFEE.to_owned(), MAX_COFFEE);
        containers.insert(WATER.to_owned(), MAX_WATER);
        containers.insert(COCOA.to_owned(), MAX_COCOA);
        containers.insert(FOAM.to_owned(), MAX_FOAM);

        Containers { all: containers }
    }

    // Gets all the ingredients for an order
    pub fn get_order(&mut self, order: &Order) {
        self.get_ingredient(&COFFEE.to_owned(), order.coffee);
        self.get_ingredient(&WATER.to_owned(), order.water);
        self.get_ingredient(&COCOA.to_owned(), order.cocoa);
        self.get_ingredient(&FOAM.to_owned(), order.foam);
    }

    // Updates the availability of the ingredients
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

mod tests {

    #[test]
    fn get_an_order_that_consumes_available_ingredients() {
        let mut containers = super::Containers::new();
        let order = crate::orders::Order::new(4, 9, 1, 1);

        containers.get_order(&order);

        let coffe_got = containers.all[&super::COFFEE.to_owned()];
        let water_got = containers.all[&super::WATER.to_owned()];
        let cocoa_got = containers.all[&super::COCOA.to_owned()];
        let foam_got = containers.all[&super::FOAM.to_owned()];

        assert_eq!(1, coffe_got);
        assert_eq!(1, water_got);
        assert_eq!(1, cocoa_got);
        assert_eq!(1, foam_got);
    }

    #[test]
    fn get_an_order_that_consumes_all_ingredients() {
        let mut containers = super::Containers::new();
        let order = crate::orders::Order::new(5, 10, 2, 2);

        containers.get_order(&order);

        let coffe_got = containers.all[&super::COFFEE.to_owned()];
        let water_got = containers.all[&super::WATER.to_owned()];
        let cocoa_got = containers.all[&super::COCOA.to_owned()];
        let foam_got = containers.all[&super::FOAM.to_owned()];

        assert_eq!(0, coffe_got);
        assert_eq!(0, water_got);
        assert_eq!(0, cocoa_got);
        assert_eq!(0, foam_got);
    }
}
