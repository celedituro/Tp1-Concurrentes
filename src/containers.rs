use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{container::Container, errors::Error};

const COFFEE: &str = "coffee";
const HOT_WATER: &str = "hot_water";
const COCOA: &str = "cocoa";
const FOAM: &str = "foam";
const GRAIN_COFFEE: &str = "grain_coffee";
const MILK: &str = "milk";
const COLD_WATER: &str = "cold_water";
const INITIAL_QUANTITY_WATER: u32 = 1000;

#[derive(Debug, Clone)]
pub struct Containers {
    pub all: HashMap<String, Arc<RwLock<Container>>>,
}

impl Containers {
    /// Creates a new [`Containers`].
    pub fn new(max_value: u32) -> Containers {
        let mut containers = HashMap::new();
        containers.insert(
            COFFEE.to_owned(),
            Arc::new(RwLock::new(Container::new(COFFEE.to_owned(), max_value))),
        );
        containers.insert(
            HOT_WATER.to_owned(),
            Arc::new(RwLock::new(Container::new(HOT_WATER.to_owned(), max_value))),
        );
        containers.insert(
            COCOA.to_owned(),
            Arc::new(RwLock::new(Container::new(COCOA.to_owned(), max_value))),
        );
        containers.insert(
            FOAM.to_owned(),
            Arc::new(RwLock::new(Container::new(FOAM.to_owned(), max_value))),
        );
        containers.insert(
            GRAIN_COFFEE.to_owned(),
            Arc::new(RwLock::new(Container::new(
                GRAIN_COFFEE.to_owned(),
                max_value,
            ))),
        );
        containers.insert(
            MILK.to_owned(),
            Arc::new(RwLock::new(Container::new(MILK.to_owned(), max_value))),
        );
        containers.insert(
            COLD_WATER.to_owned(),
            Arc::new(RwLock::new(Container::new(
                COLD_WATER.to_owned(),
                INITIAL_QUANTITY_WATER,
            ))),
        );

        Containers { all: containers }
    }

    // Calls to the container of the ingredient that receives to decrement its quantity.
    pub fn get_ingredient(
        self,
        ingredient: &String,
        value: u32,
        dispenser_id: Option<u32>,
        coffee_maker_id: u32,
    ) -> Result<(), Error> {
        if let Ok(mut container) = self.all[ingredient].write() {
            container.dispense(value, dispenser_id, coffee_maker_id)?;
        } else {
            return Err(Error::CantWriteContainerLock);
        }

        Ok(())
    }

    // Calls to the container of the ingredient that receives to increment its quantity.
    pub fn replenish_ingredient(
        self,
        ingredient: &String,
        value: u32,
        coffee_maker_id: u32,
    ) -> Result<(), Error> {
        if let Ok(mut container) = self.all[ingredient].write() {
            container.replenish(value, coffee_maker_id)?;
        } else {
            return Err(Error::CantWriteContainerLock);
        }

        Ok(())
    }

    // Gets the quantity of an ingredient
    pub fn get_quantity_of(&self, ingredient: &String) -> Result<u32, Error> {
        let quantity;
        if let Ok(container) = self.all[ingredient].read() {
            quantity = container.quantity;
        } else {
            return Err(Error::CantReadContainerLock);
        }

        Ok(quantity)
    }
}

impl Default for Containers {
    fn default() -> Self {
        Self::new(100)
    }
}
