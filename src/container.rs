use crate::errors::Error;

#[derive(Debug, Clone)]
pub struct Container {
    pub availability: u32,
}

impl Container {
    // Creates a container with its availability
    pub fn new(value: u32) -> Container {
        Container {
            availability: value,
        }
    }

    // Updates its availability
    pub fn update_availability(
        &mut self,
        value: u32,
        dispenser_id: i32,
        ingredient: &String,
    ) -> Result<(), Error> {
        println!(
            "[DISPENSER {:?}] GETTING {:?} OF {:?} FROM {:?}",
            dispenser_id, value, ingredient, self.availability
        );
        if self.availability >= value {
            self.availability -= value
        } else {
            println!(
                "[DISPENSER {:?}] ERROR NOT ENOUGH {:?}",
                dispenser_id, ingredient
            );
            return Err(Error::NotEnoughIngredient);
        };

        Ok(())
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new(0)
    }
}
