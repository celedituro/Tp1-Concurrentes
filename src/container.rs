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
    pub fn update_availability(&mut self, value: u32) -> Result<(), Error> {
        if self.availability >= value {
            self.availability -= value
        } else {
            return Err(Error::NotHaveEnoughIngredient);
        };

        Ok(())
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new(0)
    }
}
