use crate::errors::Error;

#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub quantity: u32,
}

impl Container {
    // Creates a container with its ingredient's name and quantity
    pub fn new(ingredient: String, value: u32) -> Container {
        Container {
            name: ingredient,
            quantity: value,
        }
    }

    // Updates the quantity of its ingredient if can, returns an error if not
    pub fn update_quantity(
        &mut self,
        value: u32,
        dispenser_id: i32,
        coffee_maker_id: i32,
    ) -> Result<(), Error> {
        println!(
            "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: GETTING {:?} OF {:?} FROM {:?}",
            dispenser_id, coffee_maker_id, value, self.name, self.quantity
        );
        if self.quantity >= value {
            self.quantity -= value
        } else {
            println!(
                "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: ERROR NOT ENOUGH {:?}",
                dispenser_id, coffee_maker_id, self.name
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

mod tests {

    #[test]
    fn test01_get_a_value_less_than_its_quantity_and_update_its_quantity() {
        let mut container = crate::container::Container::new("coffe".to_string(), 10);
        container.update_quantity(5, 0, 0).unwrap();
        let quantity_expected = 5;
        assert_eq!(container.quantity, quantity_expected);
    }

    #[test]
    fn test02_get_a_value_equal_than_its_quantity_and_update_its_quantity() {
        let mut container = crate::container::Container::new("coffe".to_string(), 10);
        container.update_quantity(10, 0, 0).unwrap();
        let quantity_expected = 0;
        assert_eq!(container.quantity, quantity_expected);
    }

    #[test]
    fn test03_get_a_value_greater_than_its_quantity_and_returns_an_error() {
        let mut container = crate::container::Container::new("coffe".to_string(), 10);
        let error_got = container.update_quantity(15, 0, 0).unwrap_err();
        let error_expected = crate::errors::Error::NotEnoughIngredient;
        assert_eq!(error_got, error_expected);
    }
}
