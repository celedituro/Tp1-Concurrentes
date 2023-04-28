pub mod order_handler {
    use std::sync::{Arc, Condvar, Mutex, RwLock};

    use crate::{
        coffee_maker::CoffeeMaker, dispensers::dispenser::make_order, errors::Error, orders::Order,
    };

    // Gets an order from the list of orders if there are more orders to make, returns an error if not
    fn get_order(
        orders: Arc<RwLock<Vec<Order>>>,
        dispenser_id: u32,
        coffee_maker_id: u32,
    ) -> Result<Order, Error> {
        let order = if let Ok(mut orders) = orders.write() {
            if !orders.is_empty() {
                orders.remove(0)
            } else {
                println!(
                    "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: CANT HAVE ORDERS LOCK",
                    dispenser_id, coffee_maker_id
                );
                return Err(Error::NoMoreOrders);
            }
        } else {
            return Err(Error::CantWriteOrdersLock);
        };

        Ok(order)
    }

    // Gets an order and processes it if it can, returns an error if not
    pub fn process_order(
        orders: Arc<RwLock<Vec<Order>>>,
        coffee_maker: CoffeeMaker,
        dispenser_id: u32,
        orders_processed: Arc<(Mutex<i32>, Condvar)>,
        has_to_replenish_coffee: Arc<(Mutex<bool>, Condvar)>,
    ) -> Result<(), Error> {
        loop {
            match get_order(orders.clone(), dispenser_id, coffee_maker.id) {
                Ok(order) => {
                    println!(
                        "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: MAKING {:?}",
                        dispenser_id, coffee_maker.id, order
                    );
                    make_order(
                        order,
                        coffee_maker.clone(),
                        dispenser_id,
                        orders_processed.clone(),
                        has_to_replenish_coffee.clone(),
                    )?;
                }
                Err(error) => return Err(error),
            }
        }
    }
}
