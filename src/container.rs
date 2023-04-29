use crate::errors::Error;

#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub quantity: u32,
}

impl Container {
    /// Creates a new [`Container`].
    pub fn new(ingredient: String, value: u32) -> Container {
        Container {
            name: ingredient,
            quantity: value,
        }
    }

    /// Decrements the quantity of its ingredient if can, returns an error if not.
    pub fn dispense(
        &mut self,
        value: u32,
        dispenser_id: Option<u32>,
        coffee_maker_id: u32,
    ) -> Result<(), Error> {
        println!(
            "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: GETTING {:?} OF {:?} FROM {:?}",
            dispenser_id, coffee_maker_id, value, self.name, self.quantity
        );
        if self.quantity >= value {
            self.quantity -= value
        } else {
            return Err(Error::NotEnoughIngredient);
        };

        Ok(())
    }

    // Increments the quantity of its ingredient if can, returns an error if not
    pub fn replenish(&mut self, value: u32, coffee_maker_id: u32) -> Result<(), Error> {
        println!(
            "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: INCREMENTING {:?} OF {:?} FROM {:?}",
            coffee_maker_id, value, self.name, self.quantity
        );
        self.quantity += value;

        Ok(())
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new("".to_string(), 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{container::Container, errors::Error};

    #[test]
    fn test01_get_a_value_lower_than_its_quantity_and_update_its_quantity() {
        let mut container = Container::new("coffe".to_string(), 10);
        container
            .dispense(5, Some(0), 0)
            .expect("There is not enough ingredient to make the order");
        let quantity_expected = 5;
        assert_eq!(container.quantity, quantity_expected);
    }

    #[test]
    fn test02_get_a_value_equal_than_its_quantity_and_update_its_quantity() {
        let mut container = Container::new("coffe".to_string(), 10);
        container
            .dispense(10, Some(0), 0)
            .expect("There is not enough ingredient to make the order");
        let quantity_expected = 0;
        assert_eq!(container.quantity, quantity_expected);
    }

    #[test]
    fn test03_get_a_value_greater_than_its_quantity_and_returns_an_error() {
        let mut container = Container::new("coffe".to_string(), 10);
        let error_got = container
            .dispense(15, Some(0), 0)
            .expect_err("There is not enough ingredient to make the order");
        let error_expected = Error::NotEnoughIngredient;
        assert_eq!(error_got, error_expected);
    }

    #[test]
    fn test04_increase_its_quantity_when_replenishing() {
        let mut container = Container::new("coffe".to_string(), 10);
        container.replenish(5, 0).expect("Error when replinishing");
        let quantity_expected = 15;
        assert_eq!(container.quantity, quantity_expected);
    }
}
