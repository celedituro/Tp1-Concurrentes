pub mod presenter {

    use std::{
        collections::HashMap,
        sync::{Arc, Condvar, Mutex, RwLock},
        thread,
        time::Duration,
    };

    use crate::{
        coffee_maker::CoffeeMaker,
        containers::Containers,
        errors::Error,
        orders::Order,
        stat_maker::stats_maker::{get_containers_info, get_ingredients_consumed, get_quantity_of},
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
    const VALUE_TO_ALERT: u32 = 50;

    /// Shows the current quantity of all the containers of all the coffee machines.
    pub fn present_level_of_containers(
        containers_level: Vec<HashMap<String, u32>>,
    ) -> Result<(), Error> {
        println!("\n[LEVEL OF CONTAINERS]\n");
        for (id, containers) in containers_level.iter().enumerate() {
            for ingredient in INGREDIENTS {
                let quantity = containers[&ingredient.to_owned()];
                println!(
                    "\n[{:?} CONTAINER] OF [COFFEE MACHINE {:?}]: {:?}\n",
                    ingredient, id as i32, quantity
                );
            }
        }

        Ok(())
    }

    /// Shows the current quantity of ingredients consumed between all the containers of all the
    /// coffee machines.
    pub fn present_ingredients_consumed(ingredients_consumed: HashMap<String, u32>) {
        println!("\n[INGREDIENTS CONSSUMED]\n");

        for ingredient in INGREDIENTS {
            let quantity = ingredients_consumed[&ingredient.to_owned()];
            println!("\nCONSUMPTION OF {:?}: {:?}\n", ingredient, quantity);
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
        println!("\n[TOTAL ORDERS PROCESSED]: {:?}\n", current_num_orders);
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
                println!("[PRESENTER]: PREPARING STATS",);
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
            println!("[PRESENTER]: STARTING");

            match present_statistics(coffee_makers.clone(), orders_processed.clone()) {
                Ok(_) => {
                    println!("[PRESENTER]: FINISHING");
                    thread::sleep(Duration::from_secs(3))
                }
                Err(error) => {
                    println!("[PRESENTER]: {:?}", error);
                    break;
                }
            }

            if let Ok(orders) = orders.read() {
                if orders.is_empty() {
                    println!("[PRESENTER]: FINISHING SINCE NO MORE ORDERS");
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

    /// Shows an alert if the level of grain coffee container, the milk container or the cocoa container
    /// is less than quarter of its initial capacity.
    pub fn show_alert_of_capacity(
        orders: Arc<RwLock<Vec<Order>>>,
        containers: Containers,
        coffee_maker_id: u32,
        has_to_alert: Arc<(Mutex<Vec<bool>>, Condvar)>,
        values: HashMap<i32, String>,
    ) -> Result<(), Error> {
        let handle = thread::spawn(move || loop {
            println!(
                "[ALERTER] OF [COFFEE MAKER {:?}]: STARTING",
                coffee_maker_id
            );
            if let Ok(orders) = orders.read() {
                if orders.is_empty() {
                    println!(
                        "[ALERTER] OF [COFFEE MAKER {:?}]: FINISHING SINCE NO MORE ORDERS",
                        coffee_maker_id
                    );
                    break;
                }
            }

            let (has_to_alert_lock, condvar) = &*has_to_alert;
            if let Ok(has_to_alert) = has_to_alert_lock.lock() {
                println!(
                    "[ALERTER] OF [COFFEE MAKER {:?}]: WAITING SINCE HAS TO ALERT {:?}",
                    coffee_maker_id, has_to_alert
                );
                if let Ok(mut has_to_alert) =
                    condvar.wait_while(has_to_alert, |v| v.iter().all(|&b| !b))
                {
                    println!(
                        "[ALERTER] OF [COFFEE MAKER {:?}]: PREPARING ALARM {:?}",
                        coffee_maker_id, has_to_alert
                    );
                    let containers_level = get_quantity_of(containers.clone());
                    for i in 0..3 {
                        if has_to_alert[i] {
                            let ingredient = &values[&(i as i32)];
                            let value = containers_level[ingredient];
                            if value == VALUE_TO_ALERT {
                                println!("\n[ALERTER] OF [COFFEE MAKER {:?}]: THE LEVEL OF THE CONTAINER OF {:?} IS {:?}\n", coffee_maker_id, ingredient, value);
                            }
                            has_to_alert[i] = false;
                        }
                    }
                }
            }
            condvar.notify_all();
        });

        match handle.join() {
            Ok(_) => println!(
                "[ALERTER] OF [COFFEE MAKER {:?}]: FINISHING",
                coffee_maker_id
            ),
            Err(err) => println!(
                "[ALERTER] OF [COFFEE MAKER {:?}]: {:?} WHEN JOINING",
                err, coffee_maker_id
            ),
        }

        Ok(())
    }
}
