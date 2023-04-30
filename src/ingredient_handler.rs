use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};

use crate::containers::Containers;
use crate::errors::Error;
use crate::orders_handler::order_handler::notify_to_replenish_ingredient;

const COFFEE: &str = "coffee";
const HOT_WATER: &str = "hot_water";
const FOAM: &str = "foam";
const GRAIN_COFFEE: &str = "grain_coffee";
const MILK: &str = "milk";
const COLD_WATER: &str = "cold_water";
const COCOA: &str = "cocoa";

const IDX_COFFEE: u32 = 0;
const IDX_WATER: u32 = 1;
const IDX_FOAM: u32 = 2;

#[derive(Clone)]
pub struct IHandler {
    coffee_maker_id: u32,
    containers: Containers,
    values: HashMap<String, (String, u32, u32, u32)>,
}

impl IHandler {
    /// Creates a new [`IHandler`].
    pub fn new(
        containers_list: Containers,
        id: u32,
        replenish_value: u32,
        min_value_to_replenish: u32,
    ) -> IHandler {
        let mut ingredients = HashMap::new();
        ingredients.insert(
            COFFEE.to_owned(),
            (
                GRAIN_COFFEE.to_owned(),
                replenish_value,
                min_value_to_replenish,
                IDX_COFFEE
            ),
        );
        ingredients.insert(
            FOAM.to_owned(),
            (MILK.to_owned(), replenish_value, min_value_to_replenish,
            IDX_FOAM
        ),
        );
        ingredients.insert(
            HOT_WATER.to_owned(),
            (COLD_WATER.to_owned(), replenish_value, min_value_to_replenish,
            IDX_WATER
        ),
        );

        IHandler {
            coffee_maker_id: id,
            containers: containers_list,
            values: ingredients,
        }
    }

    /// Returns true if there is not enough ingredient, false if there is.
    fn has_to_replenish(self, ingredient: &String) -> Result<bool, Error> {
        let current_quantity = self.containers.get_quantity_of(ingredient)?;
        let replenish = self.values[ingredient].2 >= current_quantity;
        println!(
            "[INGREDIENT HANDLER] IN [COFFEE MAKER {:?}]: ¿HAS TO REPLENISH {:?}? {:?}",
            self.coffee_maker_id, ingredient, replenish
        );

        Ok(replenish)
    }

    pub fn get_index(self, ingredient: String) -> u32 {
        self.values[&ingredient].3
    }

    /// Check if there is enough ingredient and notifies it to the ingredient handler.
    pub fn check_for_ingredient(
        &mut self,
        ingredient: String,
        has_to_replenish: Arc<(Mutex<Vec<bool>>, Condvar)>,
    ) -> Result<(), Error> {
        if ingredient != COCOA {
            println!(
                "[COFFEE MAKER {:?}]: CHECKING FOR {:?}",
                self.coffee_maker_id, ingredient
            );
    
            if self.clone().has_to_replenish(&ingredient)? {
                notify_to_replenish_ingredient(has_to_replenish, self.clone().get_index(ingredient));
            }
        }

        Ok(())
    }

    /// Decrease the quantity of the resource of the ingredient.
    fn get_ingredient(&mut self, ingredient: &String) -> Result<(), Error> {
        let resource = &self.values[ingredient].0;
        println!(
            "[INGREDIENT HANDLER] IN [COFFEE MAKER {:?}]: EXTRACTING FROM {:?} CONTAINER",
            self.coffee_maker_id, resource
        );
        self.containers.clone().get_ingredient(
            resource,
            self.values[ingredient].1,
            None,
            self.coffee_maker_id,
        )?;

        Ok(())
    }

    /// Increments the quantity of the ingredient.
    pub fn replenish_ingredient(&mut self, ingredient: &String) -> Result<(), Error> {
        if self.clone().has_to_replenish(ingredient)? {
            println!(
                "[INGREDIENT HANDLER] IN [COFFEE MAKER {:?}]: GETTING MORE {:?} ",
                self.coffee_maker_id, ingredient
            );
            self.containers.clone().replenish_ingredient(
                ingredient,
                self.values[ingredient].1,
                self.coffee_maker_id,
            )?;
        }

        Ok(())
    }

    /// Performs the increment and decrement of the quantities of the ingredient and its
    /// resource.
    /// If the ingredient is water, it only performs the increment of it.
    pub fn replenish(&mut self, ingredient: &String) -> Result<(), Error> {
        if self.clone().has_to_replenish(ingredient)? {
            self.get_ingredient(ingredient)?;
            self.replenish_ingredient(ingredient)?;
        }

        Ok(())
    }

    /// Performs the replenishment of the ingredient received when its necessary.
    pub fn do_replenish(
        &mut self,
        ingredient: &String,
        has_to_replenish: Arc<(Mutex<Vec<bool>>, Condvar)>,
        handler_is_awake: Arc<(Mutex<Vec<bool>>, Condvar)>,
        idx: usize,
    ) -> Result<(), Error> {
        let (handler_is_awake_lock, condvar) = &*handler_is_awake;
        if let Ok(mut handler_is_awake) = handler_is_awake_lock.lock() {
            handler_is_awake[idx] = true;
        }
        condvar.notify_all();

        let (has_to_replenish_lock, condvar) = &*has_to_replenish;
        if let Ok(has_to_replenish) = has_to_replenish_lock.lock() {
            println!(
                "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: WAITING SINCE HAS TO REPLENISH {:?} IS {:?}",
                self.coffee_maker_id, ingredient, has_to_replenish
            );
            if let Ok(mut has_to_replenish) = condvar.wait_while(has_to_replenish, |v| !v[idx]) {
                println!(
                    "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: START REPLENISHING {:?}",
                    self.coffee_maker_id, ingredient
                );
                has_to_replenish[idx] = false;
                self.replenish(ingredient)?;
            }
        }
        condvar.notify_all();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        containers::Containers,
        errors::Error,
        ingredient_handler::{COFFEE, FOAM, HOT_WATER},
    };

    use super::IHandler;

    #[test]
    fn test01_quantity_is_ten_and_replenish_value_is_ten_so_when_replenishing_them_theirs_quantity_is_twenty(
    ) {
        let mut handler = IHandler::new(Containers::new(10), 0, 10, 20);
        handler
            .replenish(&COFFEE.to_owned())
            .expect("Error when replenishing coffee");
        handler
            .replenish(&HOT_WATER.to_owned())
            .expect("Error when replenishing water");
        handler
            .replenish(&FOAM.to_owned())
            .expect("Error when replenishing foam");

        let coffee_got = handler
            .containers
            .get_quantity_of(&COFFEE.to_string())
            .expect("Error when locking coffee container");
        let foam_got = handler
            .containers
            .get_quantity_of(&FOAM.to_string())
            .expect("Error when locking foam container");
        let water_got = handler
            .containers
            .get_quantity_of(&HOT_WATER.to_string())
            .expect("Error when locking water container");

        assert_eq!(coffee_got, 20);
        assert_eq!(foam_got, 20);
        assert_eq!(water_got, 20);
    }

    #[test]
    fn test02_quantity_is_ten_and_replenish_value_is_ten_so_when_replenishing_ingredient_the_resource_quantity_is_zero(
    ) {
        let mut handler = IHandler::new(Containers::new(10), 0, 10, 20);
        handler
            .replenish(&COFFEE.to_owned())
            .expect("Error when replenishing coffee");
        handler
            .replenish(&HOT_WATER.to_owned())
            .expect("Error when replenishing water");
        handler
            .replenish(&FOAM.to_owned())
            .expect("Error when replenishing foam");

        let grain_coffee_got = handler
            .containers
            .get_quantity_of(&"grain_coffee".to_string())
            .expect("Error when locking coffee container");
        let milk_got = handler
            .containers
            .get_quantity_of(&"milk".to_string())
            .expect("Error when locking foam container");

        assert_eq!(grain_coffee_got, 0);
        assert_eq!(milk_got, 0);
    }

    #[test]
    fn test03_has_to_replenish_but_do_not_has_enough_resource_so_do_not_replenish_them() {
        let mut handler = IHandler::new(Containers::new(10), 0, 50, 20);
        let err_got = handler
            .replenish(&COFFEE.to_owned())
            .expect_err("Error when replenishing coffee");
        let err_expected = Error::NotEnoughIngredient;

        assert_eq!(err_got, err_expected);
    }

    #[test]
    fn test04_quantity_is_ten_and_min_value_to_replenish_is_ten_so_has_to_replenish() {
        let handler = IHandler::new(Containers::new(10), 0, 10, 20);
        let got = handler
            .has_to_replenish(&COFFEE.to_owned())
            .expect("Error when replenishing coffee");

        assert_eq!(got, true);
    }

    #[test]
    fn test05_quantity_is_twenty_and_min_value_to_replenish_is_ten_so_do_not_has_to_replenish() {
        let handler = IHandler::new(Containers::new(20), 0, 10, 10);
        let got = handler
            .has_to_replenish(&COFFEE.to_owned())
            .expect("Error when replenishing coffee");

        assert_eq!(got, false);
    }
}
