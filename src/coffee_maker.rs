use crate::containers::Containers;
use crate::ingredient_handler::IHandler;
use crate::orders_handler::order_handler::process_order;
use crate::{errors::Error, orders::Order};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread::{self, JoinHandle};

const DISPENSERS: u32 = 5;
// const COFFEE: &str = "coffee";
// const WATER: &str = "water";
// const FOAM: &str = "foam";
// const INGREDIENTS_TO_REPLENISH: [&str; 3] = [COFFEE, WATER, FOAM];

#[derive(Clone)]
pub struct CoffeeMaker {
    pub id: u32,
    pub containers: Containers,
    pub handler: IHandler,
}

impl CoffeeMaker {
    /// Creates a new [`CoffeeMaker`].
    pub fn new(
        id_value: u32,
        initial_quantity: u32,
        replenish_value: u32,
        min_value_to_replenish: u32,
    ) -> CoffeeMaker {
        let c = Containers::new(initial_quantity);
        CoffeeMaker {
            id: id_value,
            containers: c.clone(),
            handler: IHandler::new(c, id_value, replenish_value, min_value_to_replenish),
        }
    }

    fn handle_order(
        self,
        orders: Arc<RwLock<Vec<Order>>>,
        dispenser_id: u32,
        orders_processed: Arc<(Mutex<i32>, Condvar)>,
        has_to_replenish_coffee: Arc<(Mutex<bool>, Condvar)>,
    ) {
        match process_order(
            orders,
            self.clone(),
            dispenser_id,
            orders_processed,
            has_to_replenish_coffee,
        ) {
            Ok(_) => println!(
                "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: FINISHING",
                dispenser_id, self.id
            ),
            Err(error) => println!(
                "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: {:?}",
                dispenser_id, self.id, error
            ),
        }
    }

    /*fn handle_replenish(&mut self, ingredient: &String, has_to_replenish: Arc<(Mutex<bool>, Condvar)>) -> Result<(), Error> {
            let (has_to_replenish_lock, condvar) = &*has_to_replenish;
            if let Ok(has_to_replenish) = has_to_replenish_lock.lock() {
                println!("[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: WAITING SINCE HAS TO REPLENISH {:?}", self.id, has_to_replenish);
                if let Ok(mut has_to_replenish) = condvar.wait_while(has_to_replenish, |value| !*value) {
                    println!("[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: START REPLENISHING {:?}", self.id, ingredient);
                    match self.handler.replenish(ingredient) {
                        Ok(_) =>  {
                            *has_to_replenish = false;
                            println!("[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]]: FINISHING REPLENISHING {:?}", self.id, ingredient)},
                        Err(error) => {
                            println!("[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: ERROR {:?} WHEN REPLENISHING {:?}", self.id, error, ingredient);
                            return Err(error)
                        },
                    };
                }
            }
            condvar.notify_all();

        Ok(())
    }*/

    // Makes the dispensers to start making orders
    pub fn start(
        self,
        orders: &Arc<RwLock<Vec<Order>>>,
        orders_processed: Arc<(Mutex<i32>, Condvar)>,
    ) -> Result<(), Error> {
        let has_to_replenish_coffee: Arc<(Mutex<bool>, Condvar)> =
            Arc::new((Mutex::new(false), Condvar::new()));
        let mut dispensers: Vec<JoinHandle<()>> = Vec::new();

        for i in 0..DISPENSERS {
            let orders = Arc::clone(orders);
            let orders_processed = orders_processed.clone();
            let has_to_replenish_coffee = has_to_replenish_coffee.clone();
            let coffee_machine = self.clone();
            let handle = thread::spawn(move || {
                println!(
                    "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: STARTING",
                    i, self.id
                );
                coffee_machine.clone().handle_order(
                    orders.clone(),
                    i,
                    orders_processed,
                    has_to_replenish_coffee,
                );
            });

            dispensers.push(handle);
        }

        /* let orders = Arc::clone(&orders);
        let mut coffee_machine = self.clone();
        println!(
            "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: STARTING", self.id
        );
        let handle = thread::spawn(move || loop {
            match coffee_machine.handle_replenish(&COFFEE.to_string(), has_to_replenish_coffee.clone()) {
                Ok(_) => println!(
                    "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: FINISHING", self.id
                ),
                Err(error) => {
                    println!(
                        "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: ERROR {:?}", self.id, error
                    );
                    break
                }
            }

            if let Ok(orders) = orders.read() {
                if orders.is_empty() {
                    println!(
                        "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: NO MORE ORDERS", self.id
                    );
                    break;
                }
            }

        });

        match handle.join() {
            Ok(_) => println!(
                "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: FINISHING",
                self.id
            ),
            Err(_) => println!(
                "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: ERROR WHEN JOINING",
                self.id
            ),
        }*/

        for handle in dispensers {
            match handle.join() {
                Ok(_) => println!("[DISPENSER] OF [COFFEE MAKER {:?}]: FINISHING", self.id),
                Err(_) => println!(
                    "[DISPENSER] OF [COFFEE MAKER {:?}]: ERROR WHEN JOINING",
                    self.id
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
    fn test01_get_an_order_that_cant_be_completed() {
        let coffee_maker = CoffeeMaker::new(0, 100, 50, 20);
        let order = Order::new(110, 100, 100, 100);
        let mut orders_list = Vec::new();
        orders_list.push(order);
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));
        let has_to_replenish_coffee: Arc<(Mutex<bool>, Condvar)> =
            Arc::new((Mutex::new(false), Condvar::new()));

        let result = process_order(
            orders,
            coffee_maker,
            0,
            orders_processed,
            has_to_replenish_coffee,
        )
        .expect_err("There is not enough ingredient to make the order");
        let err_expected = Error::NotEnoughIngredient;

        assert_eq!(result, err_expected);
    }

    #[test]
    fn test02_get_an_order_when_there_are_no_orders() {
        let coffee_maker = CoffeeMaker::new(0, 100, 50, 20);
        let orders_list = Vec::new();
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));
        let has_to_replenish_coffee: Arc<(Mutex<bool>, Condvar)> =
            Arc::new((Mutex::new(false), Condvar::new()));

        let result = process_order(
            orders,
            coffee_maker,
            0,
            orders_processed,
            has_to_replenish_coffee,
        )
        .expect_err("There are no more orders");
        let err_expected = Error::NoMoreOrders;

        assert_eq!(result, err_expected);
    }

    #[test]
    fn test03_makes_one_orders_and_the_quantity_of_its_containers_get_updated() {
        let mut orders_list = Vec::new();
        let order = Order::new(10, 10, 5, 5);
        orders_list.push(order);
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let coffee_maker = CoffeeMaker::new(0, 100, 50, 20);
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
        let water_got = coffee_maker
            .containers
            .get_quantity_of(&"water".to_string())
            .expect("Error when locking water container");
        let cocoa_got = coffee_maker
            .containers
            .get_quantity_of(&"cocoa".to_string())
            .expect("Error when locking cocoa container");

        assert_eq!(coffee_got, 90);
        assert_eq!(foam_got, 95);
        assert_eq!(water_got, 90);
        assert_eq!(cocoa_got, 95);
    }

    #[test]
    fn test04_makes_five_orders_and_the_quantity_of_its_containers_get_updated() {
        let mut orders_list = Vec::new();
        let order = Order::new(10, 10, 5, 5);
        for _ in 0..5 {
            orders_list.push(order.clone());
        }
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let coffee_maker = CoffeeMaker::new(0, 100, 50, 20);
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
        let water_got = coffee_maker
            .containers
            .get_quantity_of(&"water".to_string())
            .expect("Error when locking water container");
        let cocoa_got = coffee_maker
            .containers
            .get_quantity_of(&"cocoa".to_string())
            .expect("Error when locking cocoa container");

        assert_eq!(coffee_got, 50);
        assert_eq!(water_got, 50);
        assert_eq!(cocoa_got, 75);
        assert_eq!(foam_got, 75);
    }

    // #[test]
    // fn test05_get_more_coffee_and_decrease_quantity_of_grain_coffee_container() {
    //     let mut orders_list = Vec::new();
    //     let order = Order::new(20, 10, 5, 5);
    //     for _ in 0..5 {
    //         orders_list.push(order.clone());
    //     }
    //     let orders = Arc::new(RwLock::new(orders_list));
    //     let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

    //     let coffee_maker = CoffeeMaker::new(0, 100, 50, 20);
    //     coffee_maker
    //         .clone()
    //         .start(&orders, orders_processed)
    //         .expect("Error when starting");

    //     let grain_coffee_got = coffee_maker.containers.get_quantity_of(&"grain_coffee".to_string())
    //         .expect("Error when locking coffee container");
    //     let coffee_got = coffee_maker.containers.get_quantity_of(&"coffee".to_string())
    //         .expect("Error when locking coffee container");

    //     assert_eq!(grain_coffee_got, 50);
    //     assert_eq!(coffee_got, 50);
    // }

    // #[test]
    // fn test06_get_more_foam_and_decrease_quantity_of_milk_container() {
    //     let mut orders_list = Vec::new();
    //     let order = Order::new(5, 10, 5, 10);
    //     for _ in 0..10 {
    //         orders_list.push(order.clone());
    //     }
    //     let orders = Arc::new(RwLock::new(orders_list));
    //     let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

    //     let coffee_maker = CoffeeMaker::new(0, 100, 25, 50);
    //     coffee_maker
    //         .clone()
    //         .start(&orders, orders_processed)
    //         .expect("Error when starting");

    //     let milk = coffee_maker.clone().containers.all["milk"]
    //         .read()
    //         .expect("Cant have read lock of the milk container")
    //         .quantity;
    //     assert_eq!(milk, 50);
    //     let foam: u32 = coffee_maker.clone().containers.all["foam"]
    //         .read()
    //         .expect("Cant have read lock of the foam container")
    //         .quantity;
    //     assert_eq!(foam, 50);
    // }
}
