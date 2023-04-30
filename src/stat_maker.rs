pub mod stats_maker {

    use std::collections::HashMap;

    use crate::{coffee_maker::CoffeeMaker, containers::Containers};

    const INGREDIENTS: [&str; 6] = ["coffee", "water", "cocoa", "foam", "grain_coffee", "milk"];
    const COFFEE: &str = "coffee";
    const FOAM: &str = "foam";
    const GRAIN_COFFEE: &str = "grain_coffee";
    const MILK: &str = "milk";

    /// Gets the current quantity of all the containers of all the coffee machines.
    fn get_quantity_of(containers: Containers) -> HashMap<String, u32> {
        let mut level_of_containers = HashMap::new();
        for ingredient in INGREDIENTS {
            if let Ok(quantity) = containers.clone().get_quantity_of(&ingredient.to_owned()) {
                level_of_containers.insert(ingredient.to_owned(), quantity);
            }
        }

        level_of_containers
    }

    /// Returns a list of hashmaps. Every element of the list represents a coffee machine
    /// and every hashmap represents its container where the key is the ingredient of it and
    /// the value is its quantity.
    pub fn get_containers_info(coffee_makers: Vec<CoffeeMaker>) -> Vec<HashMap<String, u32>> {
        let mut vec = Vec::new();
        for coffee_maker in coffee_makers {
            vec.push(get_quantity_of(coffee_maker.containers));
        }

        vec
    }

    fn update_replenishing_ingredient(
        mut ingredients_consumed: HashMap<String, u32>,
        ingredient: String,
        resource: String,
    ) -> HashMap<String, u32> {
        if ingredients_consumed[&resource] > 0 {
            println!("distinto de cero {}", ingredients_consumed[&resource]);
            let value = ingredients_consumed[&ingredient];
            let updated_value = value + ingredients_consumed[&resource];
            ingredients_consumed.insert(ingredient, updated_value);
        };

        ingredients_consumed
    }

    /// Shows the current quantity of ingredients consumed between all the containers of all the
    /// coffee machines.
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
        println!("ingredients consumed before: {:?}", ingredients_consumed);

        ingredients_consumed = update_replenishing_ingredient(
            ingredients_consumed.clone(),
            COFFEE.to_owned(),
            GRAIN_COFFEE.to_owned(),
        );
        ingredients_consumed = update_replenishing_ingredient(
            ingredients_consumed.clone(),
            FOAM.to_owned(),
            MILK.to_owned(),
        );

        println!("ingredients consumed after: {:?}", ingredients_consumed);
        ingredients_consumed
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Condvar, Mutex, RwLock},
        thread::{self, JoinHandle},
    };

    use crate::{
        coffee_maker::CoffeeMaker,
        orders::Order,
        stat_maker::stats_maker::{get_containers_info, get_ingredients_consumed},
    };

    #[test]
    fn test01_get_one_coffee_maker_with_no_orders_processed() {
        let mut coffee_makers = Vec::new();
        coffee_makers.push(CoffeeMaker::new(0, 100, 50, 10));
        let orders_list = Vec::new();
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let coffee_maker = coffee_makers[0].clone();
        coffee_maker
            .start(&orders, orders_processed.clone())
            .expect("Error when coffee machine 0 process order");
        let (orders_processed_lock, _condvar) = &*orders_processed;
        if let Ok(orders_processed) = orders_processed_lock.lock() {
            assert_eq!(*orders_processed, 0);
        }
        let containers_level_got = get_containers_info(coffee_makers);
        assert_eq!(containers_level_got[0]["coffee"], 100);
        assert_eq!(containers_level_got[0]["water"], 100);
        assert_eq!(containers_level_got[0]["foam"], 100);
        assert_eq!(containers_level_got[0]["cocoa"], 100);
        assert_eq!(containers_level_got[0]["grain_coffee"], 100);
        assert_eq!(containers_level_got[0]["milk"], 100);
    }

    #[test]
    fn test02_get_one_coffee_maker_with_one_order_and_the_level_of_its_container_is_updated() {
        let mut coffee_makers = Vec::new();
        coffee_makers.push(CoffeeMaker::new(0, 100, 50, 10));
        let mut orders_list = Vec::new();
        orders_list.push(Order::new(10, 10, 5, 5));
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let coffee_maker = coffee_makers[0].clone();
        coffee_maker
            .start(&orders, orders_processed.clone())
            .expect("Error when coffee machine 0 process order");

        let (orders_processed_lock, _condvar) = &*orders_processed;
        if let Ok(orders_processed) = orders_processed_lock.lock() {
            assert_eq!(*orders_processed, 1);
        }
        let containers_level_got = get_containers_info(coffee_makers);
        assert_eq!(containers_level_got[0]["coffee"], 90);
        assert_eq!(containers_level_got[0]["water"], 90);
        assert_eq!(containers_level_got[0]["foam"], 95);
        assert_eq!(containers_level_got[0]["cocoa"], 95);
        assert_eq!(containers_level_got[0]["grain_coffee"], 100);
        assert_eq!(containers_level_got[0]["milk"], 100);
    }

    #[test]
    fn test03_get_one_coffee_maker_with_five_orders_and_the_level_of_its_container_is_updated() {
        let mut coffee_makers = Vec::new();
        coffee_makers.push(CoffeeMaker::new(0, 100, 50, 10));
        let mut orders_list = Vec::new();
        let order = Order::new(10, 10, 5, 5);
        for _ in 0..5 {
            orders_list.push(order.clone());
        }
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let coffee_maker = coffee_makers[0].clone();
        coffee_maker
            .start(&orders, orders_processed.clone())
            .expect("Error when coffee machine 0 process order");

        let (orders_processed_lock, _condvar) = &*orders_processed;
        if let Ok(orders_processed) = orders_processed_lock.lock() {
            assert_eq!(*orders_processed, 5);
        }
        let containers_level_got = get_containers_info(coffee_makers);
        assert_eq!(containers_level_got[0]["coffee"], 50);
        assert_eq!(containers_level_got[0]["water"], 50);
        assert_eq!(containers_level_got[0]["foam"], 75);
        assert_eq!(containers_level_got[0]["cocoa"], 75);
        assert_eq!(containers_level_got[0]["grain_coffee"], 100);
        assert_eq!(containers_level_got[0]["milk"], 100);
    }

    #[test]
    fn test04_get_one_coffee_maker_with_ten_orders_and_the_level_of_its_container_is_updated() {
        let mut coffee_makers = Vec::new();
        coffee_makers.push(CoffeeMaker::new(0, 100, 50, 10));
        let mut orders_list = Vec::new();
        let order = Order::new(10, 10, 5, 5);
        for _ in 0..10 {
            orders_list.push(order.clone());
        }
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let coffee_maker = coffee_makers[0].clone();
        coffee_maker
            .start(&orders, orders_processed.clone())
            .expect("Error when coffee machine 0 process order");

        let (orders_processed_lock, _condvar) = &*orders_processed;
        if let Ok(orders_processed) = orders_processed_lock.lock() {
            assert_eq!(*orders_processed, 10);
        }
        let containers_level_got = get_containers_info(coffee_makers);
        assert_eq!(containers_level_got[0]["coffee"], 50);
        assert_eq!(containers_level_got[0]["water"], 50);
        assert_eq!(containers_level_got[0]["foam"], 50);
        assert_eq!(containers_level_got[0]["cocoa"], 50);
        assert_eq!(containers_level_got[0]["grain_coffee"], 50);
        assert_eq!(containers_level_got[0]["milk"], 100);
    }

    #[test]
    fn test05_get_one_coffee_maker_with_no_orders_and_ingredients_consumed_are_zero() {
        let mut coffee_makers = Vec::new();
        coffee_makers.push(CoffeeMaker::new(0, 100, 50, 20));
        let orders_list = Vec::new();
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let coffee_maker = coffee_makers[0].clone();
        coffee_maker
            .start(&orders, orders_processed.clone())
            .expect("Error when coffee machine 0 process order");
        let (orders_processed_lock, _condvar) = &*orders_processed;
        if let Ok(orders_processed) = orders_processed_lock.lock() {
            assert_eq!(*orders_processed, 0);
        }
        let containers_level = get_containers_info(coffee_makers);
        let ingredients_consumed_got = get_ingredients_consumed(containers_level, 100);
        assert_eq!(ingredients_consumed_got["coffee"], 0);
        assert_eq!(ingredients_consumed_got["water"], 0);
        assert_eq!(ingredients_consumed_got["foam"], 0);
        assert_eq!(ingredients_consumed_got["cocoa"], 0);
        assert_eq!(ingredients_consumed_got["grain_coffee"], 0);
        assert_eq!(ingredients_consumed_got["milk"], 0);
    }

    #[test]
    fn test06_get_one_coffee_maker_with_orders_processed_and_ingredients_consumed_are_updated() {
        let mut coffee_makers = Vec::new();
        coffee_makers.push(CoffeeMaker::new(0, 100, 50, 10));
        let mut orders_list = Vec::new();
        orders_list.push(Order::new(10, 10, 5, 5));
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let coffee_maker = coffee_makers[0].clone();
        coffee_maker
            .start(&orders, orders_processed.clone())
            .expect("Error when coffee machine 0 process order");

        let containers_level = get_containers_info(coffee_makers);
        let ingredients_consumed_got = get_ingredients_consumed(containers_level, 100);
        assert_eq!(ingredients_consumed_got["coffee"], 10);
        assert_eq!(ingredients_consumed_got["water"], 10);
        assert_eq!(ingredients_consumed_got["foam"], 5);
        assert_eq!(ingredients_consumed_got["cocoa"], 5);
        assert_eq!(ingredients_consumed_got["grain_coffee"], 0);
        assert_eq!(ingredients_consumed_got["milk"], 0);
    }

    #[test]
    fn test07_get_one_coffee_maker_with_five_orders_and_ingredients_consumed_are_updated() {
        let mut coffee_makers = Vec::new();
        coffee_makers.push(CoffeeMaker::new(0, 100, 50, 10));
        let mut orders_list = Vec::new();
        let order = Order::new(10, 10, 5, 5);
        for _ in 0..5 {
            orders_list.push(order.clone());
        }
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let coffee_maker = coffee_makers[0].clone();
        coffee_maker
            .start(&orders, orders_processed.clone())
            .expect("Error when coffee machine 0 process order");

        let containers_level = get_containers_info(coffee_makers);
        let ingredients_consumed_got = get_ingredients_consumed(containers_level, 100);
        assert_eq!(ingredients_consumed_got["coffee"], 50);
        assert_eq!(ingredients_consumed_got["water"], 50);
        assert_eq!(ingredients_consumed_got["foam"], 25);
        assert_eq!(ingredients_consumed_got["cocoa"], 25);
        assert_eq!(ingredients_consumed_got["grain_coffee"], 0);
        assert_eq!(ingredients_consumed_got["milk"], 0);
    }

    #[test]
    fn test08_get_two_coffee_makers_with_ten_orders_and_ingredients_consumed_are_updated() {
        let mut coffee_makers = Vec::new();
        coffee_makers.push(CoffeeMaker::new(0, 100, 50, 10));
        coffee_makers.push(CoffeeMaker::new(1, 100, 50, 10));
        let mut orders_list = Vec::new();
        let order = Order::new(10, 10, 5, 5);
        for _ in 0..10 {
            orders_list.push(order.clone());
        }
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let mut machines: Vec<JoinHandle<()>> = Vec::new();
        for coffee_maker in coffee_makers.clone() {
            let orders = orders.clone();
            let orders_processed = orders_processed.clone();
            let handle = thread::spawn(move || {
                let coffee_maker_clone = coffee_maker.clone();
                match coffee_maker_clone.start(&orders, orders_processed) {
                    Ok(_) => println!("[COFFEE MAKER {:?}]: FINISHING", coffee_maker.id),
                    Err(err) => {
                        println!("[COFFEE MAKER {:?}]: {:?} ERROR", coffee_maker.id, err)
                    }
                }
            });
            machines.push(handle);
        }

        for handle in machines {
            handle.join().expect("Error when joining");
        }

        let containers_level = get_containers_info(coffee_makers);
        let ingredients_consumed_got = get_ingredients_consumed(containers_level, 2 * 100);
        assert_eq!(ingredients_consumed_got["coffee"], 100);
        //assert_eq!(ingredients_consumed_got["water"], 100);
        assert_eq!(ingredients_consumed_got["foam"], 50);
        assert_eq!(ingredients_consumed_got["cocoa"], 50);
    }
}
