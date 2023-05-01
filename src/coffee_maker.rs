use crate::containers::Containers;
use crate::ingredient_handler::IHandler;
use crate::orders_handler::order_handler::process_order;
use crate::stats_presenter::presenter::show_alert_of_capacity;
use crate::{errors::Error, orders::Order};
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread::{self, JoinHandle};

const DISPENSERS: u32 = 2;
const COFFEE: &str = "coffee";
const FOAM: &str = "foam";
const HOT_WATER: &str = "hot_water";
const INGREDIENTS_TO_REPLENISH: [&str; 3] = [COFFEE, FOAM, HOT_WATER];

#[derive(Clone)]
pub struct CoffeeMaker {
    pub id: u32,
    pub containers: Containers,
    pub handler: IHandler,
}

impl CoffeeMaker {
    /// Creates a new [`CoffeeMaker`].
    pub fn new(id_value: u32, initial_quantity: u32, replenish_value: u32) -> CoffeeMaker {
        let c = Containers::new(initial_quantity);
        CoffeeMaker {
            id: id_value,
            containers: c.clone(),
            handler: IHandler::new(c, id_value, replenish_value),
        }
    }

    /// Calls to the ingredient handler to perform the replenishing of ingredients.
    fn handle_replenish(
        self,
        orders: Arc<RwLock<Vec<Order>>>,
        has_to_replenish: Arc<(Mutex<Vec<bool>>, Condvar)>,
        handler_is_awake: Arc<(Mutex<Vec<bool>>, Condvar)>,
        has_to_alert: Arc<(Mutex<Vec<bool>>, Condvar)>,
    ) -> Result<(), Error> {
        println!(
            "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: STARTING",
            self.id
        );

        let mut handlers: Vec<JoinHandle<Result<(), Error>>> = Vec::new();
        for (i, ingredient) in INGREDIENTS_TO_REPLENISH.iter().enumerate() {
            let mut coffee_maker = self.clone();
            let has_to_replenish = has_to_replenish.clone();
            let handler_is_awake = handler_is_awake.clone();
            let has_to_alert = has_to_alert.clone();

            let orders = orders.clone();
            let handle: JoinHandle<Result<(), Error>> = thread::spawn(move || loop {
                match coffee_maker.handler.do_replenish(
                    &ingredient.to_string(),
                    has_to_replenish.clone(),
                    handler_is_awake.clone(),
                    i,
                    has_to_alert.clone(),
                ) {
                    Ok(_) => {
                        println!(
                            "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: FINISHING",
                            self.id
                        );
                    }
                    Err(err) => {
                        println!(
                            "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: {:?}",
                            self.id, err
                        );
                        return Err(err);
                    }
                }

                if let Ok(orders) = orders.read() {
                    if orders.is_empty() {
                        return Err(Error::NoMoreOrders);
                    }
                }
            });
            handlers.push(handle)
        }

        let values = self.clone().get_values();
        show_alert_of_capacity(orders, self.containers, self.id, has_to_alert, values)?;

        for handle in handlers {
            match handle.join() {
                Ok(_) => println!(
                    "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: FINISHING",
                    self.id
                ),
                Err(err) => println!(
                    "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: {:?}",
                    self.id, err
                ),
            }
        }

        Ok(())
    }

    fn get_values(self) -> HashMap<i32, String> {
        let mut values = HashMap::new();
        values.insert(0, "grain_coffee".to_string());
        values.insert(1, "milk".to_string());
        values.insert(2, "cocoa".to_string());

        values
    }

    /// Makes its dispensers to start making orders.
    pub fn start(
        self,
        orders: &Arc<RwLock<Vec<Order>>>,
        orders_processed: Arc<(Mutex<i32>, Condvar)>,
    ) -> Result<(), Error> {
        let has_to_replenish: Arc<(Mutex<Vec<bool>>, Condvar)> =
            Arc::new((Mutex::new(vec![false, false, false]), Condvar::new()));
        let handler_is_awake: Arc<(Mutex<Vec<bool>>, Condvar)> =
            Arc::new((Mutex::new(vec![false, false, false]), Condvar::new()));
        let has_to_alert: Arc<(Mutex<Vec<bool>>, Condvar)> =
            Arc::new((Mutex::new(vec![false, false, false]), Condvar::new()));

        let mut dispensers: Vec<JoinHandle<Result<(), Error>>> = Vec::new();
        for i in 0..DISPENSERS {
            let orders = Arc::clone(orders);
            let orders_processed = orders_processed.clone();
            let coffee_machine = self.clone();

            let has_to_replenish = has_to_replenish.clone();
            let handler_is_awake = handler_is_awake.clone();
            let has_to_alert = has_to_alert.clone();

            let handle = thread::spawn(move || {
                let (handle_is_awake_lock, condvar) = &*handler_is_awake;
                if let Ok(handler_is_awake) = handle_is_awake_lock.lock() {
                    println!(
                        "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: WAITING SINCE HANDLER IS NOT AWAKE",
                        i, self.id
                    );
                    if condvar
                        .wait_while(handler_is_awake, |v| v.iter().any(|&b| !b))
                        .is_ok()
                    {
                        println!(
                            "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: STARTING SINCE HANDLER IS AWAKE",
                            i, self.id
                        );
                        process_order(
                            orders,
                            coffee_machine,
                            i,
                            orders_processed,
                            has_to_replenish,
                            has_to_alert,
                        )?;
                    }
                }

                Ok(())
            });

            dispensers.push(handle);
        }

        self.clone().handle_replenish(
            Arc::clone(orders),
            has_to_replenish,
            handler_is_awake,
            has_to_alert,
        )?;

        for handle in dispensers {
            match handle.join() {
                Ok(_) => println!("[DISPENSER] OF [COFFEE MAKER {:?}]: FINISHING", self.id),
                Err(err) => println!(
                    "[DISPENSER] OF [COFFEE MAKER {:?}]: {:?} WHEN JOINING",
                    err, self.id
                ),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Condvar, Mutex, RwLock};

    use crate::errors::Error;
    use crate::orders_handler::order_handler::process_order;
    use crate::{coffee_maker::CoffeeMaker, orders::Order};

    #[test]
    fn test01_get_an_order_when_there_are_no_orders() {
        let coffee_maker = CoffeeMaker::new(0, 100, 50);
        let orders_list = Vec::new();
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));
        let has_to_replenish: Arc<(Mutex<Vec<bool>>, Condvar)> =
            Arc::new((Mutex::new(vec![false, false, false]), Condvar::new()));
        let has_to_alert: Arc<(Mutex<Vec<bool>>, Condvar)> =
            Arc::new((Mutex::new(vec![false, false, false]), Condvar::new()));

        let result = process_order(
            orders,
            coffee_maker,
            0,
            orders_processed,
            has_to_replenish,
            has_to_alert,
        )
        .expect_err("There are no more orders");
        let err_expected = Error::NoMoreOrders;

        assert_eq!(result, err_expected);
    }

    #[test]
    fn test02_makes_one_orders_and_the_quantity_of_its_containers_get_updated() {
        let mut orders_list = Vec::new();
        let order = Order::new(10, 10, 5, 5);
        orders_list.push(order);
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let coffee_maker = CoffeeMaker::new(0, 100, 50);
        coffee_maker
            .clone()
            .start(&orders, orders_processed)
            .expect("Error when starting");

        let coffee_got = coffee_maker
            .containers
            .get_quantity_of(&"coffee".to_string())
            .expect("Error when locking coffee container");
        let foam_got = coffee_maker
            .containers
            .get_quantity_of(&"foam".to_string())
            .expect("Error when locking foam container");
        let hot_water_got = coffee_maker
            .containers
            .get_quantity_of(&"hot_water".to_string())
            .expect("Error when locking hot water container");
        let cocoa_got = coffee_maker
            .containers
            .get_quantity_of(&"cocoa".to_string())
            .expect("Error when locking cocoa container");

        assert_eq!(coffee_got, 90);
        assert_eq!(foam_got, 95);
        assert_eq!(hot_water_got, 90);
        assert_eq!(cocoa_got, 95);
    }

    #[test]
    fn test03_makes_five_orders_and_the_quantity_of_its_containers_get_updated() {
        let mut orders_list = Vec::new();
        let order = Order::new(10, 10, 5, 5);
        for _ in 0..5 {
            orders_list.push(order.clone());
        }
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let coffee_maker = CoffeeMaker::new(0, 100, 50);
        coffee_maker
            .clone()
            .start(&orders, orders_processed)
            .expect("Error when starting");

        let coffee_got = coffee_maker
            .containers
            .get_quantity_of(&"coffee".to_string())
            .expect("Error when locking coffee container");
        let foam_got = coffee_maker
            .containers
            .get_quantity_of(&"foam".to_string())
            .expect("Error when locking foam container");
        let hot_water_got = coffee_maker
            .containers
            .get_quantity_of(&"hot_water".to_string())
            .expect("Error when locking hot water container");
        let cocoa_got = coffee_maker
            .containers
            .get_quantity_of(&"cocoa".to_string())
            .expect("Error when locking cocoa container");

        assert_eq!(coffee_got, 50);
        assert_eq!(hot_water_got, 50);
        assert_eq!(cocoa_got, 75);
        assert_eq!(foam_got, 75);
    }

    #[test]
    fn test04_get_more_coffee_and_decrease_quantity_of_grain_coffee_container() {
        let mut orders_list = Vec::new();
        let order = Order::new(10, 10, 5, 5);
        for _ in 0..15 {
            orders_list.push(order.clone());
        }
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let coffee_maker = CoffeeMaker::new(0, 100, 50);
        coffee_maker
            .clone()
            .start(&orders, orders_processed)
            .expect("Error when starting");

        let grain_coffee_got = coffee_maker
            .containers
            .get_quantity_of(&"grain_coffee".to_string())
            .expect("Error when locking coffee container");

        assert_eq!(grain_coffee_got, 50);
    }

    #[test]
    fn test05_get_more_foam_and_decrease_quantity_of_milk_container() {
        let mut orders_list = Vec::new();
        let order = Order::new(5, 10, 5, 10);
        for _ in 0..15 {
            orders_list.push(order.clone());
        }
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let coffee_maker = CoffeeMaker::new(0, 100, 50);
        coffee_maker
            .clone()
            .start(&orders, orders_processed)
            .expect("Error when starting");

        let milk_got = coffee_maker.clone().containers.all["milk"]
            .read()
            .expect("Cant have read lock of the milk container")
            .quantity;

        assert_eq!(milk_got, 50);
    }
}
