pub mod coffe_maker {
    use crate::{containers::Containers, errors::Error, orders::Order};

    const COFFEE: &str = "coffee";
    const WATER: &str = "water";
    const COCOA: &str = "cocoa";
    const FOAM: &str = "foam";

    // Gets the ingredients of an order
    pub fn get_ingredient(
        mut containers: Containers,
        ingredient: &String,
        value: u32,
    ) -> Result<(), Error> {
        if let Some(c) = containers.all.get_mut(ingredient) {
            c.update_availability(value)?;
        }

        Ok(())
    }

    // Gets all the ingredients for an order
    pub fn get_order(containers: Containers, order: &Order) -> Result<(), Error> {
        get_ingredient(containers.clone(), &COFFEE.to_owned(), order.coffee)?;
        get_ingredient(containers.clone(), &WATER.to_owned(), order.water)?;
        get_ingredient(containers.clone(), &COCOA.to_owned(), order.cocoa)?;
        get_ingredient(containers, &FOAM.to_owned(), order.foam)?;

        Ok(())
    }
}

mod tests {

    #[test]
    fn test01_get_an_order_that_cant_be_completed() {
        let containers = crate::containers::Containers::new();
        let order = crate::orders::Order::new(6, 10, 2, 2);

        let result = crate::coffee_maker::coffe_maker::get_order(containers, &order).unwrap_err();
        let err_expected = crate::errors::Error::NotHaveEnoughIngredient;

        assert_eq!(result, err_expected);
    }
}
