pub mod presenter {

    use std::{
        collections::HashMap,
        sync::{Arc, Condvar, Mutex, RwLock},
        thread,
        time::Duration,
    };

    use crate::{
        coffee_maker::CoffeeMaker,
        errors::Error,
        orders::Order,
        stat_maker::stats_maker::{get_containers_info, get_ingredients_consumed},
    };

    const INGREDIENTS: [&str; 6] = [
        "coffee",
        "hot_water",
        "cocoa",
        "foam",
        "grain_coffee",
        "milk",
    ];
    const INITIAL_QUANTITY: u32 = 100;

    /// Shows the current quantity of all the containers of all the coffee machines.
    pub fn present_level_of_containers(
        containers_level: Vec<HashMap<String, u32>>,
    ) -> Result<(), Error> {
        println!("[LEVEL OF CONTAINERS]");
        for (id, containers) in containers_level.iter().enumerate() {
            for ingredient in INGREDIENTS {
                let quantity = containers[&ingredient.to_owned()];
                println!(
                    "[{:?} CONTAINER] OF [COFFEE MACHINE {:?}]: {:?}",
                    ingredient, id as i32, quantity
                );
            }
        }

        Ok(())
    }

    /// Shows the current quantity of ingredients consumed between all the containers of all the
    /// coffee machines.
    pub fn present_ingredients_consumed(ingredients_consumed: HashMap<String, u32>) {
        println!("[INGREDIENTS CONSSUMED]");

        for ingredient in INGREDIENTS {
            let quantity = ingredients_consumed[&ingredient.to_owned()];
            println!("CONSUMPTION OF {:?}: {:?}", ingredient, quantity);
        }
    }

    /// Shows stats of the level of containers of all the coffee machines,
    /// the total of orders processed and the total of ingredients consumed between
    /// all the containers of all the coffee machines.
    pub fn present_stats(
        coffee_makers: Vec<CoffeeMaker>,
        current_num_orders: u32,
        initial_quantity: u32,
    ) -> Result<(), Error> {
        let containers_level = get_containers_info(coffee_makers);
        present_level_of_containers(containers_level.clone())?;
        println!("[TOTAL ORDERS PROCESSED]: {:?}", current_num_orders);
        let ingredients_consumed = get_ingredients_consumed(containers_level, initial_quantity);
        present_ingredients_consumed(ingredients_consumed);

        Ok(())
    }

    /// Handles the presentation of stats periodically.
    fn present_statistics(
        coffee_makers: Vec<CoffeeMaker>,
        orders_processed: Arc<(Mutex<i32>, Condvar)>,
    ) -> Result<(), Error> {
        let (orders_processed_lock, condvar) = &*orders_processed;
        if let Ok(orders_processed) = orders_processed_lock.lock() {
            println!("[PRESENTER]: WAITING");
            if let Ok(orders_processed) = condvar.wait_while(orders_processed, |num| *num == 0) {
                println!("PRESENTING STATS WITH NUM ORDERS: {:?}", orders_processed);
                present_stats(
                    coffee_makers.clone(),
                    *orders_processed as u32,
                    INITIAL_QUANTITY * coffee_makers.len() as u32,
                )?;
            }
        }
        condvar.notify_all();

        Ok(())
    }

    /// Performs the presentation of statistics until there are no more orders to make.
    pub fn show_statistics(
        coffee_makers: Vec<CoffeeMaker>,
        orders_processed: Arc<(Mutex<i32>, Condvar)>,
        orders: Arc<RwLock<Vec<Order>>>,
    ) -> Result<(), Error> {
        let presenter_handle = thread::spawn(move || loop {
            println!("[PRESENTER]: PREPARING STATS");
            if let Ok(orders) = orders.read() {
                if orders.is_empty() {
                    println!("[PRESENTER]: FINISHING SINCE NO MORE ORDERS");
                    break;
                }
            }
            match present_statistics(coffee_makers.clone(), orders_processed.clone()) {
                Ok(_) => thread::sleep(Duration::from_secs(3)),
                Err(error) => {
                    println!("[PRESENTER]: {:?}", error);
                    break;
                }
            }
        });

        match presenter_handle.join() {
            Ok(_) => println!("[PRESENTER]: FINISHING"),
            Err(_) => println!("[PRESENTER]: ERROR WHEN JOINING"),
        };

        Ok(())
    }
}
