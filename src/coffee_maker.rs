pub mod coffe_maker {
    use std::sync::{Arc, RwLock};

    use crate::{containers::Containers, errors::Error, orders::Order};

    // Gets an order from the list of orders if there are more orders to make, returns an error if not
    pub fn get_order(
        orders: Arc<RwLock<Vec<Order>>>,
        mut containers: Containers,
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
            println!("[DISPENSER {:?}] MAKING {:?}", dispenser_id, order);
            containers.get_ingredients(order, dispenser_id)?;
        } else {
            return Err(Error::NoMoreOrders);
        }

        Ok(())
    }
}

mod tests {

    #[test]
    fn test01_get_an_order_that_cant_be_completed() {
        let containers = crate::containers::Containers::new();
        let order = crate::orders::Order::new(110, 100, 100, 100);
        let mut vec = Vec::new();
        vec.push(order);
        let orders = std::sync::Arc::new(std::sync::RwLock::new(vec));

        let result =
            crate::coffee_maker::coffe_maker::get_order(orders, containers, 0).unwrap_err();
        let err_expected = crate::errors::Error::NotEnoughIngredient;

        assert_eq!(result, err_expected);
    }

    #[test]
    fn test02_get_an_order_when_there_are_no_orders() {
        let containers = crate::containers::Containers::new();
        let vec = Vec::new();
        let orders = std::sync::Arc::new(std::sync::RwLock::new(vec));

        let result =
            crate::coffee_maker::coffe_maker::get_order(orders, containers, 0).unwrap_err();
        let err_expected = crate::errors::Error::NoMoreOrders;

        assert_eq!(result, err_expected);
    }
}
