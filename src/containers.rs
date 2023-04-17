use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{container::Container, errors::Error, orders::Order};

const MAX_COFFEE: u32 = 100;
const MAX_WATER: u32 = 100;
const MAX_COCOA: u32 = 100;
const MAX_FOAM: u32 = 100;

const COFFEE: &str = "coffee";
const WATER: &str = "water";
const COCOA: &str = "cocoa";
const FOAM: &str = "foam";

#[derive(Debug, Clone)]
pub struct Containers {
    pub all: HashMap<String, Arc<RwLock<Container>>>,
}

impl Containers {
    // Creates a hashmap of containers of ingredients
    pub fn new() -> Containers {
        let mut containers = HashMap::new();
        containers.insert(
            COFFEE.to_owned(),
            Arc::new(RwLock::new(Container::new(COFFEE.to_owned(), MAX_COFFEE))),
        );
        containers.insert(
            WATER.to_owned(),
            Arc::new(RwLock::new(Container::new(WATER.to_owned(), MAX_WATER))),
        );
        containers.insert(
            COCOA.to_owned(),
            Arc::new(RwLock::new(Container::new(COCOA.to_owned(), MAX_COCOA))),
        );
        containers.insert(
            FOAM.to_owned(),
            Arc::new(RwLock::new(Container::new(FOAM.to_owned(), MAX_FOAM))),
        );

        Containers { all: containers }
    }

    // Gets the ingredients of an order
    pub fn get_ingredient(
        &mut self,
        ingredient: &String,
        value: u32,
        dispenser_id: i32,
    ) -> Result<(), Error> {
        if let Some(c) = self.all.get_mut(ingredient) {
            {
                let mut container = c.write().unwrap();
                container.update_quantity(value, dispenser_id)?;
            }
        }

        println!("[DISPENSER {:?}] GOT {:?}", dispenser_id, ingredient);

        Ok(())
    }

    pub fn get_ingredients(&mut self, order: Order, dispenser_id: i32) -> Result<(), Error> {
        println!("[DISPENSER {:?}] GETTING COFFEE", dispenser_id);
        self.get_ingredient(&COFFEE.to_owned(), order.coffee, dispenser_id)?;
        println!("[DISPENSER {:?}] GETTING WATER", dispenser_id);
        self.get_ingredient(&WATER.to_owned(), order.water, dispenser_id)?;
        println!("[DISPENSER {:?}] GETTING COCOA", dispenser_id);
        self.get_ingredient(&COCOA.to_owned(), order.cocoa, dispenser_id)?;
        println!("[DISPENSER {:?}] GETTING FOAM", dispenser_id);
        self.get_ingredient(&FOAM.to_owned(), order.foam, dispenser_id)?;

        println!("[DISPENSER {:?}] GOT ALL INGREDIENTS", dispenser_id);
        Ok(())
    }
}

impl Default for Containers {
    fn default() -> Self {
        Self::new()
    }
}
