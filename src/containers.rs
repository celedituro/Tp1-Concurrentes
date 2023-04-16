use std::collections::HashMap;

use crate::container::Container;

const MAX_COFFEE: u32 = 5;
const MAX_WATER: u32 = 10;
const MAX_COCOA: u32 = 2;
const MAX_FOAM: u32 = 2;

const COFFEE: &str = "coffee";
const WATER: &str = "water";
const COCOA: &str = "cocoa";
const FOAM: &str = "foam";

#[derive(Debug, Clone)]
pub struct Containers {
    pub all: HashMap<String, Container>,
}

impl Containers {
    // Creates a hashmap of containers of ingredients
    pub fn new() -> Containers {
        let mut containers = HashMap::<String, Container>::new();
        containers.insert(COFFEE.to_owned(), Container::new(MAX_COFFEE));
        containers.insert(WATER.to_owned(), Container::new(MAX_WATER));
        containers.insert(COCOA.to_owned(), Container::new(MAX_COCOA));
        containers.insert(FOAM.to_owned(), Container::new(MAX_FOAM));

        Containers { all: containers }
    }
}

impl Default for Containers {
    fn default() -> Self {
        Self::new()
    }
}
