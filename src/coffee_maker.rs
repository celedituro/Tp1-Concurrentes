use crate::containers::Containers;
use crate::{errors::Error, orders::Order};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct CoffeeMaker {
    id: i32,
    containers: Containers,
}

impl CoffeeMaker {
    pub fn new(id_value: i32) -> CoffeeMaker {
        CoffeeMaker {
            id: id_value,
            containers: Containers::new(),
        }
    }

    // Gets an order from the list of orders if there are more orders to make, returns an error if not
    pub fn get_order(
        mut self,
        orders: Arc<RwLock<Vec<Order>>>,
        dispenser_id: i32,
    ) -> Result<(), Error> {
        let order: Option<Order> = if let Ok(mut orders) = orders.write() {
            if !orders.is_empty() {
                Some(orders.remove(0))
            } else {
                return Err(Error::NoMoreOrders);
            }
        } else {
            return Err(Error::NoMoreOrders);
        };

        if let Some(order) = order {
            println!(
                "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: MAKING {:?}",
                dispenser_id, self.id, order
            );
            self.containers
                .get_ingredients(order, dispenser_id, self.id)?;
        } else {
            return Err(Error::NoMoreOrders);
        }

        Ok(())
    }
}

mod tests {

    #[test]
    fn test01_get_an_order_that_cant_be_completed() {
        let coffee_maker = crate::coffee_maker::CoffeeMaker::new(1);
        let order = crate::orders::Order::new(110, 100, 100, 100);
        let mut vec = Vec::new();
        vec.push(order);
        let orders = std::sync::Arc::new(std::sync::RwLock::new(vec));

        let result = coffee_maker.get_order(orders, 0).unwrap_err();
        let err_expected = crate::errors::Error::NotEnoughIngredient;

        assert_eq!(result, err_expected);
    }

    #[test]
    fn test02_get_an_order_when_there_are_no_orders() {
        let coffee_maker = crate::coffee_maker::CoffeeMaker::new(1);
        let vec = Vec::new();
        let orders = std::sync::Arc::new(std::sync::RwLock::new(vec));

        let result = coffee_maker.get_order(orders, 0).unwrap_err();
        let err_expected = crate::errors::Error::NoMoreOrders;

        assert_eq!(result, err_expected);
    }
}
