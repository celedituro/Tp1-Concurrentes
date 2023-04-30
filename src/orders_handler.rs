pub mod order_handler {
    use std::sync::{Arc, Condvar, Mutex, RwLock};

    use crate::{
        coffee_maker::CoffeeMaker, dispensers::dispenser::make_order, errors::Error, orders::Order,
    };

    const IDX_COFFEE: u32 = 0;
    const IDX_WATER: u32 = 1;
    const IDX_FOAM: u32 = 2;

    /// Notifies to replenish an ingredient.
    pub fn notify_to_replenish(has_to_replenish: Arc<(Mutex<Vec<bool>>, Condvar)>) {
        let (has_to_replenish_lock, condvar) = &*has_to_replenish;
        if let Ok(mut has_to_replenish) = has_to_replenish_lock.lock() {
            has_to_replenish[IDX_COFFEE as usize] = true;
            has_to_replenish[IDX_FOAM as usize] = true;
            has_to_replenish[IDX_WATER as usize] = true;
        }
        condvar.notify_all();
    }

    /// Notifies to replenish an ingredient.
    pub fn notify_to_replenish_ingredient(
        has_to_replenish: Arc<(Mutex<Vec<bool>>, Condvar)>,
        idx: u32,
    ) {
        let (has_to_replenish_lock, condvar) = &*has_to_replenish;
        if let Ok(mut has_to_replenish) = has_to_replenish_lock.lock() {
            has_to_replenish[idx as usize] = true;
        }
        condvar.notify_all();
    }

    /// Gets an order from the list of orders if there are more orders to make,
    /// returns an error if not.
    fn get_order(
        orders: Arc<RwLock<Vec<Order>>>,
        has_to_replenish: Arc<(Mutex<Vec<bool>>, Condvar)>,
    ) -> Result<Order, Error> {
        let order = if let Ok(mut orders) = orders.write() {
            if !orders.is_empty() {
                orders.remove(0)
            } else {
                notify_to_replenish(has_to_replenish);

                return Err(Error::NoMoreOrders);
            }
        } else {
            return Err(Error::CantWriteOrdersLock);
        };

        Ok(order)
    }

    /// Gets an order and processes it if it can,
    /// returns an error if not.
    pub fn process_order(
        orders: Arc<RwLock<Vec<Order>>>,
        coffee_maker: CoffeeMaker,
        dispenser_id: u32,
        orders_processed: Arc<(Mutex<i32>, Condvar)>,
        has_to_replenish: Arc<(Mutex<Vec<bool>>, Condvar)>,
    ) -> Result<(), Error> {
        loop {
            match get_order(orders.clone(), has_to_replenish.clone()) {
                Ok(order) => {
                    println!(
                        "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: MAKING {:?}",
                        dispenser_id, coffee_maker.id, order
                    );
                    match make_order(
                        order,
                        coffee_maker.clone(),
                        dispenser_id,
                        orders_processed.clone(),
                        has_to_replenish.clone(),
                    ) {
                        Ok(_) => println!(
                            "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: FINISHING ORDER",
                            dispenser_id, coffee_maker.id
                        ),
                        Err(err) => match err {
                            Error::NotEnoughIngredient => {
                                println!(
                                    "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: {:?}",
                                    dispenser_id, coffee_maker.id, err
                                );
                                continue;
                            }
                            _ => return Err(err),
                        },
                    };
                }
                Err(err) => return Err(err),
            }
        }
    }
}
