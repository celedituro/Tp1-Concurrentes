pub mod presenter {

    use std::{
        collections::HashMap,
        sync::{Arc, Condvar, Mutex, RwLock},
        thread,
        time::Duration,
    };

    use crate::{coffee_maker::CoffeeMaker, containers::Containers, errors::Error, orders::Order};

    const INGREDIENTS: [&str; 6] = ["coffee", "water", "cocoa", "foam", "grain_coffee", "milk"];
    const INITIAL_QUANTITY: u32 = 100;

    /// Gets the current quantity of all the containers of all the coffee machines
    pub fn get_quantity_of(containers: Containers) -> HashMap<String, u32> {
        let mut level_of_containers = HashMap::new();
        for ingredient in INGREDIENTS {
            if let Ok(quantity) = containers.clone().get_quantity_of(&ingredient.to_owned()) {
                level_of_containers.insert(ingredient.to_owned(), quantity);
            }
        }

        level_of_containers
    }

    pub fn get_containers_info(coffee_makers: Vec<CoffeeMaker>) -> Vec<HashMap<String, u32>> {
        let mut vec = Vec::new();
        for coffee_maker in coffee_makers {
            vec.push(get_quantity_of(coffee_maker.containers));
        }

        vec
    }

    /// Shows the current quantity of ingredients consumed between all the containers of all the
    /// coffee machines-
    pub fn get_ingredients_consumed(
        containers_level: Vec<HashMap<String, u32>>,
        initial_quantity: u32,
    ) -> HashMap<String, u32> {
        let mut ingredients_consumed = HashMap::new();
        for ingredient in INGREDIENTS {
            let mut current = 0;
            for i in containers_level.clone() {
                current += i[&ingredient.to_owned()];
            }
            ingredients_consumed.insert(ingredient.to_owned(), initial_quantity - current);
        }

        ingredients_consumed
    }

    /// Shows the current quantity of all the containers of all the coffee machines
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
                if quantity == 0 {
                    return Err(Error::NoMoreIngredient);
                }
            }
        }

        Ok(())
    }

    /// Shows the current quantity of ingredients consumed between all the containers of all the
    /// coffee machines
    pub fn present_ingredients_consumed(ingredients_consumed: HashMap<String, u32>) {
        println!("[INGREDIENTS CONSSUMED]");

        for ingredient in INGREDIENTS {
            let quantity = ingredients_consumed[&ingredient.to_owned()];
            println!("CONSUMPTION OF {:?}: {:?}", ingredient, quantity);
        }
    }

    /// Shows stats of the level of containers of all the coffee machines, the total of orders processed
    /// and the total of ingredients consumed between all the containers of all the coffee machines
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
                Ok(_) => thread::sleep(Duration::from_secs(1)),
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
