use crate::containers::Containers;
use crate::errors::Error;

const COFFEE: &str = "coffee";
const GRAIN_COFFEE: &str = "grain_coffee";
const MIN_COFFEE: u32 = 20;

#[derive(Clone)]
pub struct CoffeeGrinder {
    coffee_maker_id: i32,
    containers: Containers,
}

impl CoffeeGrinder {
    pub fn new(containers_list: Containers, id: i32) -> CoffeeGrinder {
        CoffeeGrinder {
            coffee_maker_id: id,
            containers: containers_list,
        }
    }

    // Returns false if there is enough coffee, returns true if not
    fn has_to_grind_coffee(&self) -> Result<bool, Error> {
        let has_to_grind;
        if let Ok(container) = self.containers.all[&COFFEE.to_owned()].read() {
            has_to_grind = container.quantity < MIN_COFFEE;
        } else {
            return Err(Error::CantWriteContainerLock);
        };

        Ok(has_to_grind)
    }

    // Increments the quantity of coffee and decrements the quantity of grain coffee of when there is not enough coffee
    pub fn grind_coffee(&mut self) -> Result<(), Error> {
        if let Ok(grind) = self.has_to_grind_coffee() {
            if grind {
                self.containers.get_ingredient(
                    &GRAIN_COFFEE.to_owned(),
                    50,
                    1,
                    self.coffee_maker_id,
                    false,
                )?;
                self.containers.get_ingredient(
                    &COFFEE.to_owned(),
                    50,
                    1,
                    self.coffee_maker_id,
                    true,
                )?;
            }
        };

        Ok(())
    }
}
