pub mod dispenser {
    use std::sync::{Arc, Condvar, Mutex};

    use crate::{
        coffee_maker::CoffeeMaker, errors::Error, orders::Order,
        orders_handler::order_handler::notify_to_replenish,
    };

    const COFFEE: &str = "coffee";
    const WATER: &str = "water";
    const COCOA: &str = "cocoa";
    const FOAM: &str = "foam";

    /// Increments the total num of orders processed and notifies it.
    pub fn notify_one_order_has_been_processed(
        orders_processed: Arc<(Mutex<i32>, Condvar)>,
        dispenser_id: u32,
        coffee_maker_id: u32,
    ) -> Result<(), Error> {
        let (orders_processed_lock, condvar) = &*orders_processed;
        if let Ok(mut num_orders_processed) = orders_processed_lock.lock() {
            *num_orders_processed += 1;
            println!(
                "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: GOT ALL INGREDIENTS - NUM ORDERS PROCESSED: {:?}",
                dispenser_id, coffee_maker_id, num_orders_processed
            );
        } else {
            return Err(Error::CantHaveOrdersProcessedLock);
        }

        condvar.notify_all();
        Ok(())
    }

    /// Gets all the ingredients of the order.
    /// Also calls to the ingredient handler of its coffee machine to replenish
    /// ingredients if its necessary.
    pub fn make_order(
        order: Order,
        mut coffee_maker: CoffeeMaker,
        dispenser_id: u32,
        orders_processed: Arc<(Mutex<i32>, Condvar)>,
        has_to_replenish_coffee: Arc<(Mutex<bool>, Condvar)>,
        has_to_replenish_foam: Arc<(Mutex<bool>, Condvar)>,
    ) -> Result<(), Error> {
        match coffee_maker.containers.clone().get_ingredient(
            &COFFEE.to_owned(),
            order.coffee,
            Some(dispenser_id),
            coffee_maker.id,
        ) {
            Ok(_) => {
                println!(
                    "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: GOT COFFEE",
                    dispenser_id, coffee_maker.id
                );
                coffee_maker
                    .handler
                    .check_for_ingredient(COFFEE.to_owned(), has_to_replenish_coffee)?;
            }

            Err(err) => match err {
                Error::NotEnoughIngredient => notify_to_replenish(has_to_replenish_coffee),
                _ => return Err(err),
            },
        };

        coffee_maker.containers.clone().get_ingredient(
            &WATER.to_owned(),
            order.water,
            Some(dispenser_id),
            coffee_maker.id,
        )?;

        match coffee_maker.containers.clone().get_ingredient(
            &FOAM.to_owned(),
            order.foam,
            Some(dispenser_id),
            coffee_maker.id,
        ) {
            Ok(_) => {
                println!(
                    "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: GOT COFFEE",
                    dispenser_id, coffee_maker.id
                );
                coffee_maker
                    .handler
                    .check_for_ingredient(FOAM.to_owned(), has_to_replenish_foam)?;
            }
            Err(err) => match err {
                Error::NotEnoughIngredient => notify_to_replenish(has_to_replenish_foam),
                _ => return Err(err),
            },
        }

        coffee_maker.containers.get_ingredient(
            &COCOA.to_owned(),
            order.cocoa,
            Some(dispenser_id),
            coffee_maker.id,
        )?;

        notify_one_order_has_been_processed(orders_processed, dispenser_id, coffee_maker.id)?;

        Ok(())
    }
}
