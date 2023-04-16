pub mod coffe_maker {
    use std::sync::{Arc, RwLock};

    use crate::{containers::Containers, errors::Error, orders::Order};

    // Gets an order from the vector of order
    pub fn get_order(
        orders: Arc<RwLock<Vec<Order>>>,
        containers: Arc<RwLock<Containers>>,
        dispenser_id: i32,
    ) -> Result<(), Error> {
        let mut orders = orders.write().unwrap();
        if !orders.is_empty() {
            let order = orders.remove(0);
            println!("[DISPENSER {:?}] MAKING {:?}", dispenser_id, order);
            let mut containers = containers.write().unwrap();
            containers.get_ingredients(order, dispenser_id)?;
        } else {
            println!(
                "[DISPENSER {:?}] FINALIZING, THERE ARE NO MORE ORDERS",
                dispenser_id
            );
        }

        Ok(())
    }
}

mod tests {

    #[test]
    fn test01_get_an_order_that_cant_be_completed() {
        let containers =
            std::sync::Arc::new(std::sync::RwLock::new(crate::containers::Containers::new()));
        let order = crate::orders::Order::new(6, 10, 2, 2);
        let mut vec = Vec::new();
        vec.push(order);
        println!("vec: {:?}", &vec);
        let orders = std::sync::Arc::new(std::sync::RwLock::new(vec));

        let result =
            crate::coffee_maker::coffe_maker::get_order(orders, containers, 0).unwrap_err();
        let err_expected = crate::errors::Error::NotEnoughIngredient;

        assert_eq!(result, err_expected);
    }
}
