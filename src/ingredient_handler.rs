use std::collections::HashMap;

use crate::containers::Containers;
use crate::errors::Error;

const COFFEE: &str = "coffee";
const FOAM: &str = "foam";
const GRAIN_COFFEE: &str = "grain_coffee";
const MILK: &str = "milk";
const WATER: &str = "water";

#[derive(Clone)]
pub struct IHandler {
    coffee_maker_id: i32,
    containers: Containers,
    values: HashMap<String, (String, u32, u32)>,
}

impl IHandler {
    // Creates an ingredient IHandler for replenishing ingredients
    pub fn new(
        containers_list: Containers,
        id: i32,
        min_value_to_replanish: u32,
        replanish_value: u32,
    ) -> IHandler {
        let mut ingredients = HashMap::new();
        ingredients.insert(
            COFFEE.to_owned(),
            (
                GRAIN_COFFEE.to_owned(),
                min_value_to_replanish,
                replanish_value,
            ),
        );
        ingredients.insert(
            FOAM.to_owned(),
            (MILK.to_owned(), min_value_to_replanish, replanish_value),
        );
        ingredients.insert(
            WATER.to_owned(),
            ("".to_owned(), min_value_to_replanish, replanish_value),
        );

        IHandler {
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
            has_to_get_more = container.quantity < self.values[ingredient].1;
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
                    self.values[&ingredient].2,
                    dispenser_id,
                    self.coffee_maker_id,
                    true,
                )?;
                if ingredient != WATER {
                    println!(
                        "[DISPENSER {:?}]: GETTING MORE {:?}",
                        dispenser_id, self.values[&ingredient].0
                    );
                    self.containers.get_ingredient(
                        &self.values[&ingredient].0,
                        self.values[&ingredient].2,
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

#[cfg(test)]
mod tests {
    use crate::{containers::Containers, errors::Error};

    use super::IHandler;

    #[test]
    fn test01_quantity_is_ten_and_min_value_to_replanish_is_twenty_so_has_to_replanish() {
        let handler = IHandler::new(Containers::new(10), 0, 20, 50);
        let coffee_got = handler
            .has_to_replenish(&"coffee".to_string(), 0)
            .expect("Error");
        let foam_got = handler
            .has_to_replenish(&"foam".to_string(), 0)
            .expect("Error");
        let water_got = handler
            .has_to_replenish(&"water".to_string(), 0)
            .expect("Error");

        assert_eq!(coffee_got, true);
        assert_eq!(foam_got, true);
        assert_eq!(water_got, true);
    }

    #[test]
    fn test02_quantity_is_thirty_and_min_value_to_replenish_is_twenty_so_has_not_to_replanish() {
        let handler = IHandler::new(Containers::new(30), 0, 20, 50);
        let coffee_got = handler
            .has_to_replenish(&"coffee".to_string(), 0)
            .expect("Error");
        let foam_got = handler
            .has_to_replenish(&"foam".to_string(), 0)
            .expect("Error");
        let water_got = handler
            .has_to_replenish(&"water".to_string(), 0)
            .expect("Error");

        assert_eq!(coffee_got, false);
        assert_eq!(foam_got, false);
        assert_eq!(water_got, false);
    }

    #[test]
    fn test03_quantity_is_ten_and_min_value_to_replanish_is_twenty_so_replanish_them() {
        let mut handler = IHandler::new(Containers::new(10), 0, 20, 10);
        handler
            .replenish_ingredients(0)
            .expect("Error when replenishing ingredients");
        let coffee_got = handler.containers.all["coffee"]
            .read()
            .expect("Error when reading coffee container")
            .quantity;
        let foam_got = handler.containers.all["foam"]
            .read()
            .expect("Error when reading coffee container")
            .quantity;
        let water_got = handler.containers.all["water"]
            .read()
            .expect("Error when reading coffee container")
            .quantity;

        assert_eq!(coffee_got, 20);
        assert_eq!(foam_got, 20);
        assert_eq!(water_got, 20);
    }

    #[test]
    fn test04_quantity_is_thirty_and_min_value_to_replanish_is_twenty_so_dont_replanish_them() {
        let mut handler = IHandler::new(Containers::new(30), 0, 20, 10);
        handler
            .replenish_ingredients(0)
            .expect("Error when replenishing ingredients");
        let coffee_got = handler.containers.all["coffee"]
            .read()
            .expect("Error when reading coffee container")
            .quantity;
        let foam_got = handler.containers.all["foam"]
            .read()
            .expect("Error when reading coffee container")
            .quantity;
        let water_got = handler.containers.all["water"]
            .read()
            .expect("Error when reading coffee container")
            .quantity;

        assert_eq!(coffee_got, 30);
        assert_eq!(foam_got, 30);
        assert_eq!(water_got, 30);
    }

    #[test]
    fn test05_has_to_replanish_but_dont_have_enough_resource_so_dont_replanish_them() {
        let mut handler = IHandler::new(Containers::new(10), 0, 30, 50);
        let err_got = handler
            .replenish_ingredients(0)
            .expect_err("Dont have enough ingredient");
        let err_expected = Error::NotEnoughIngredient;
        assert_eq!(err_got, err_expected);
    }
}
