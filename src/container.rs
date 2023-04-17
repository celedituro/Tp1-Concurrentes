use crate::errors::Error;

#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub quantity: u32,
}

impl Container {
    // Creates a container with its quantity
    pub fn new(ingredient: String, value: u32) -> Container {
        Container {
            name: ingredient,
            quantity: value,
        }
    }

    // Updates its quantity
    pub fn update_quantity(&mut self, value: u32, dispenser_id: i32) -> Result<(), Error> {
        println!(
            "[DISPENSER {:?}] GETTING {:?} OF {:?} FROM {:?}",
            dispenser_id, value, self.name, self.quantity
        );
        if self.quantity >= value {
            self.quantity -= value
        } else {
            println!(
                "[DISPENSER {:?}] ERROR NOT ENOUGH {:?}",
                dispenser_id, self.name
            );
            return Err(Error::NotEnoughIngredient);
        };

        Ok(())
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new("".to_string(), 0)
    }
}
