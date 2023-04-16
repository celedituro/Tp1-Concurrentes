use crate::{containers::Containers, errors::Error, orders::Order};

const COFFEE: &str = "coffee";
const WATER: &str = "water";
const COCOA: &str = "cocoa";
const FOAM: &str = "foam";

pub struct CoffeeMaker {
    id: u32,
}

impl CoffeeMaker {
    // Creates a coffe maker with its id
    pub fn new(id_value: u32) -> CoffeeMaker {
        CoffeeMaker { id: id_value }
    }

    // Gets all the ingredients for an order
    pub fn get_order(&mut self, containers: Containers, order: &Order) -> Result<(), Error> {
        self.get_ingredient(containers.clone(), &COFFEE.to_owned(), order.coffee)?;
        self.get_ingredient(containers.clone(), &WATER.to_owned(), order.water)?;
        self.get_ingredient(containers.clone(), &COCOA.to_owned(), order.cocoa)?;
        self.get_ingredient(containers.clone(), &FOAM.to_owned(), order.foam)?;

        Ok(())
    }

    // Gets all the ingredients of an order
    pub fn get_ingredient(
        &mut self,
        mut containers: Containers,
        ingredient: &String,
        value: u32,
    ) -> Result<(), Error> {
        if let Some(container) = containers.all.get_mut(ingredient) {
            container.update_availability(value)?;
        }

        Ok(())
    }
}

impl Default for CoffeeMaker {
    fn default() -> Self {
        Self::new(0)
    }
}

mod tests {
    #[test]
    fn test01_get_an_order_that_cant_be_completed() {
        let containers = super::Containers::new();
        let mut coffee_maker = super::CoffeeMaker::new(1);
        let order = crate::orders::Order::new(6, 10, 2, 2);

        let result = coffee_maker.get_order(containers, &order).unwrap_err();
        let err_expected = crate::errors::Error::NotHaveEnoughIngredient;

        assert_eq!(result, err_expected);
    }
}
