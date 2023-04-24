use std::collections::HashMap;

use crate::containers::Containers;
use crate::errors::Error;

const COFFEE: &str = "coffee";
const FOAM: &str = "foam";
const GRAIN_COFFEE: &str = "grain_coffee";
const MILK: &str = "milk";
const WATER: &str = "water";

const MIN_VALUE: u32 = 20;
const VALUE: u32 = 50;

#[derive(Clone)]
pub struct Stocker {
    coffee_maker_id: i32,
    containers: Containers,
    values: HashMap<String, String>,
}

impl Stocker {
    // Creates an ingredient stocker for replenishing ingredients
    pub fn new(containers_list: Containers, id: i32) -> Stocker {
        let mut ingredients = HashMap::new();
        ingredients.insert(COFFEE.to_owned(), GRAIN_COFFEE.to_owned());
        ingredients.insert(FOAM.to_owned(), MILK.to_owned());

        Stocker {
            coffee_maker_id: id,
            containers: containers_list,
            values: ingredients,
        }
    }

    // Replenish coffee, foam and water if its necessary
    pub fn replenish_ingredients(&mut self, dispenser_id: i32) -> Result<(), Error> {
        println!("[DISPENSER {:?}]: GETTING MORE INGREDIENTS", dispenser_id);
        self.replenish(COFFEE.to_owned(), dispenser_id)?;
        self.replenish(FOAM.to_owned(), dispenser_id)?;
        self.replenish(WATER.to_owned(), dispenser_id)?;

        Ok(())
    }

    // Returns false if there is enough ingredient, returns true if not
    fn has_to_replenish(&self, ingredient: &String, dispenser_id: i32) -> Result<bool, Error> {
        let has_to_get_more;
        if let Ok(container) = self.containers.all[ingredient].read() {
            has_to_get_more = container.quantity < MIN_VALUE;
            println!(
                "[DISPENSER {:?}]: ¿HAS TO GET MORE {:?}? {:?}",
                dispenser_id, ingredient, has_to_get_more
            );
        } else {
            return Err(Error::CantWriteContainerLock);
        };

        Ok(has_to_get_more)
    }

    // Replenish the ingredient and dispense its given resource when there is not enough ingredient.
    // If the ingredient is water, it only replenish water
    pub fn replenish(&mut self, ingredient: String, dispenser_id: i32) -> Result<(), Error> {
        if let Ok(get_more) = self.has_to_replenish(&ingredient, dispenser_id) {
            if get_more {
                println!(
                    "[DISPENSER {:?}]: GETTING MORE{:?}",
                    dispenser_id, ingredient
                );
                self.containers.get_ingredient(
                    &ingredient,
                    VALUE,
                    dispenser_id,
                    self.coffee_maker_id,
                    true,
                )?;
                if ingredient != WATER {
                    println!(
                        "[DISPENSER {:?}]: GETTING MORE {:?}",
                        dispenser_id, self.values[&ingredient]
                    );
                    self.containers.get_ingredient(
                        &self.values[&ingredient],
                        VALUE,
                        dispenser_id,
                        self.coffee_maker_id,
                        false,
                    )?;
                } else {
                    println!("[DISPENSER {:?}]: HEATING WATER", dispenser_id);
                }
            }
        };

        Ok(())
    }
}
