pub mod dispenser {
    use std::{
        collections::HashMap,
        sync::{Arc, Condvar, Mutex},
    };

    use crate::{
        coffee_maker::CoffeeMaker, errors::Error, orders::Order,
        orders_handler::order_handler::notify_to_replenish_ingredient,
    };

    const COFFEE: &str = "coffee";
    const HOT_WATER: &str = "hot_water";
    const COCOA: &str = "cocoa";
    const FOAM: &str = "foam";

    const INGREDIENTS: [&str; 4] = [COFFEE, HOT_WATER, COCOA, FOAM];

    /// Increments the total num of orders processed and notifies it.
    pub fn notify_one_order_processed(
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

    fn convert_to_hash(order: Order) -> HashMap<String, u32> {
        let mut hash_order: HashMap<String, u32> = HashMap::new();
        hash_order.insert(COFFEE.to_owned(), order.coffee);
        hash_order.insert(FOAM.to_owned(), order.foam);
        hash_order.insert(HOT_WATER.to_owned(), order.water);
        hash_order.insert(COCOA.to_owned(), order.cocoa);

        hash_order
    }

    /// Gets all the ingredients of the order.
    /// Also calls to the ingredient handler of its coffee machine to replenish
    /// ingredients if its necessary.
    pub fn make_order(
        order: Order,
        coffee_maker: CoffeeMaker,
        dispenser_id: u32,
        orders_processed: Arc<(Mutex<i32>, Condvar)>,
        has_to_replenish: Arc<(Mutex<Vec<bool>>, Condvar)>,
    ) -> Result<(), Error> {
        let hash_order: HashMap<String, u32> = convert_to_hash(order);
        for ingredient in INGREDIENTS {
            let has_to_replenish = has_to_replenish.clone();
            match coffee_maker.containers.clone().get_ingredient(
                &ingredient.to_owned(),
                hash_order[ingredient],
                Some(dispenser_id),
                coffee_maker.id,
            ) {
                Ok(_) => {
                    println!(
                        "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: GOT {:?}",
                        dispenser_id, coffee_maker.id, ingredient
                    );
                }
                Err(err) => match err {
                    Error::NotEnoughIngredient => {
                        if ingredient != COCOA {
                            let idx = coffee_maker
                                .clone()
                                .handler
                                .get_index(ingredient.to_owned());

                            notify_to_replenish_ingredient(has_to_replenish.clone(), idx);
                            println!(
                                "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: TRY GET {:?} AGAIN",
                                dispenser_id, coffee_maker.id, ingredient
                            );
                            coffee_maker.containers.clone().get_ingredient(
                                &ingredient.to_owned(),
                                hash_order[ingredient],
                                Some(dispenser_id),
                                coffee_maker.id,
                            )?
                        }
                    }
                    _ => return Err(err),
                },
            };
        }

        notify_one_order_processed(orders_processed, dispenser_id, coffee_maker.id)?;

        Ok(())
    }
}
