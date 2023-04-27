use std::{
    collections::HashMap,
    sync::{Arc, Condvar, Mutex},
};

use crate::{container::Container, errors::Error};

const COFFEE: &str = "coffee";
const WATER: &str = "water";
const COCOA: &str = "cocoa";
const FOAM: &str = "foam";
const GRAIN_COFFEE: &str = "grain_coffee";
const MILK: &str = "milk";

#[derive(Debug, Clone)]
pub struct Containers {
    pub all: HashMap<String, Arc<(Mutex<Container>, Condvar)>>,
}

impl Containers {
    // Creates a hashmap of containers of ingredients
    pub fn new(max_value: u32) -> Containers {
        let mut containers = HashMap::new();
        containers.insert(
            COFFEE.to_owned(),
            Arc::new((
                Mutex::new(Container::new(COFFEE.to_owned(), max_value)),
                Condvar::new(),
            )),
        );
        containers.insert(
            WATER.to_owned(),
            Arc::new((
                Mutex::new(Container::new(WATER.to_owned(), max_value)),
                Condvar::new(),
            )),
        );
        containers.insert(
            COCOA.to_owned(),
            Arc::new((
                Mutex::new(Container::new(COCOA.to_owned(), max_value)),
                Condvar::new(),
            )),
        );
        containers.insert(
            FOAM.to_owned(),
            Arc::new((
                Mutex::new(Container::new(FOAM.to_owned(), max_value)),
                Condvar::new(),
            )),
        );
        containers.insert(
            GRAIN_COFFEE.to_owned(),
            Arc::new((
                Mutex::new(Container::new(GRAIN_COFFEE.to_owned(), max_value)),
                Condvar::new(),
            )),
        );
        containers.insert(
            MILK.to_owned(),
            Arc::new((
                Mutex::new(Container::new(MILK.to_owned(), max_value)),
                Condvar::new(),
            )),
        );

        Containers { all: containers }
    }

    // Gets the ingredients of an order
    pub fn get_ingredient(
        self,
        ingredient: &String,
        value: u32,
        dispenser_id: u32,
        coffee_maker_id: u32,
    ) -> Result<(), Error> {
        let (container_lock, condvar) = &*self.all[ingredient];
        if let Ok(mut container) = container_lock.lock() {
            container.dispense(value, dispenser_id, coffee_maker_id)?;
        } else {
            println!(
                "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: CANT HAVE CONTAINERS LOCK",
                dispenser_id, coffee_maker_id
            );
            return Err(Error::CantHaveContainerLock);
        }
        condvar.notify_all();

        Ok(())
    }

    // Gets the quantity of an ingredient
    pub fn get_quantity_of(&self, ingredient: &String) -> Result<u32, Error> {
        let quantity;
        let (container_lock, condvar) = &*self.all[ingredient];
        if let Ok(container) = container_lock.lock() {
            quantity = container.quantity;
        } else {
            return Err(Error::CantHaveContainerLock);
        }
        condvar.notify_all();
        println!("LIBERATING CONTAINER LOCK");
        Ok(quantity)
    }
}

impl Default for Containers {
    fn default() -> Self {
        Self::new(100)
    }
}
