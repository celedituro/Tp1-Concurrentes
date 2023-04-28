pub mod dispenser {
    use std::sync::{Arc, Condvar, Mutex};

    use crate::{containers::Containers, errors::Error, orders::Order};

    const COFFEE: &str = "coffee";
    const WATER: &str = "water";
    const COCOA: &str = "cocoa";
    const FOAM: &str = "foam";

    pub fn make_order(
        order: Order,
        containers: Containers,
        dispenser_id: u32,
        coffee_maker_id: u32,
        orders_processed: Arc<(Mutex<i32>, Condvar)>,
    ) -> Result<(), Error> {
        containers.clone().get_ingredient(
            &COFFEE.to_owned(),
            order.coffee,
            Some(dispenser_id),
            coffee_maker_id,
        )?;

        containers.clone().get_ingredient(
            &WATER.to_owned(),
            order.water,
            Some(dispenser_id),
            coffee_maker_id,
        )?;

        containers.clone().get_ingredient(
            &FOAM.to_owned(),
            order.foam,
            Some(dispenser_id),
            coffee_maker_id,
        )?;

        containers.get_ingredient(
            &COCOA.to_owned(),
            order.cocoa,
            Some(dispenser_id),
            coffee_maker_id,
        )?;

        let (orders_processed_lock, condvar) = &*orders_processed;
        if let Ok(mut num_orders_processed) = orders_processed_lock.lock() {
            *num_orders_processed += 1;
            println!(
                "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: GOT ALL INGREDIENTS - NUM ORDERS PROCESSED: {:?}",
                dispenser_id, coffee_maker_id, num_orders_processed
            );
        }
        condvar.notify_one();

        Ok(())
    }
}
