use crate::containers::Containers;
use crate::ingredient_handler::IHandler;
use crate::orders_handler::order_handler::process_order;
use crate::{errors::Error, orders::Order};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread::{self, JoinHandle};

const DISPENSERS: u32 = 5;
const COFFEE: &str = "coffee";
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

    /// Calls to the ingredient handler to perform the replenishing of ingredients.
    fn handle_replenish(
        mut self,
        orders: Arc<RwLock<Vec<Order>>>,
        has_to_replenish_coffee: Arc<(Mutex<bool>, Condvar)>,
    ) -> Result<(), Error> {
        println!(
            "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: STARTING",
            self.id
        );
        let handle = thread::spawn(move || loop {
            if let Ok(orders) = orders.read() {
                if orders.is_empty() {
                    println!(
                        "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: NO MORE ORDERS",
                        self.id
                    );
                    break;
                }
            }

            match self
                .handler
                .do_replenish(&COFFEE.to_string(), has_to_replenish_coffee.clone())
            {
                Ok(_) => println!(
                    "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: FINISHING",
                    self.id
                ),
                Err(error) => {
                    println!(
                        "[INGREDIENT HANDLER] OF [COFFEE MAKER {:?}]: ERROR {:?}",
                        self.id, error
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
        }

        Ok(())
    }

    /// Makes its dispensers to start making orders.
    pub fn start(
        self,
        orders: &Arc<RwLock<Vec<Order>>>,
        orders_processed: Arc<(Mutex<i32>, Condvar)>,
    ) -> Result<(), Error> {
        let has_to_replenish_coffee: Arc<(Mutex<bool>, Condvar)> =
            Arc::new((Mutex::new(false), Condvar::new()));

        let mut dispensers: Vec<JoinHandle<Result<(), Error>>> = Vec::new();
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
                match process_order(
                    orders,
                    coffee_machine,
                    i,
                    orders_processed,
                    has_to_replenish_coffee,
                ) {
                    Ok(_) => println!(
                        "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: ALREADY MADE ORDER",
                        i, self.id
                    ),
                    Err(err) => return Err(err),
                }

                Ok(())
            });

            dispensers.push(handle);
        }

        self.clone()
            .handle_replenish(Arc::clone(orders), has_to_replenish_coffee)?;

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
    fn test02_makes_one_orders_and_the_quantity_of_its_containers_get_updated() {
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

        let _coffee_got = coffee_maker
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

        //assert_eq!(coffee_got, 90);
        assert_eq!(foam_got, 95);
        assert_eq!(water_got, 90);
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

        let coffee_maker = CoffeeMaker::new(0, 100, 50, 20);
        coffee_maker
            .clone()
            .start(&orders, orders_processed)
            .expect("Error when starting");

        let _coffee_got = coffee_maker
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

        //assert_eq!(coffee_got, 50);
        assert_eq!(water_got, 50);
        assert_eq!(cocoa_got, 75);
        assert_eq!(foam_got, 75);
    }

    // #[test]
    // fn test05_get_more_coffee_and_decrease_quantity_of_grain_coffee_container() {
    //     let mut orders_list = Vec::new();
    //     let order = Order::new(20, 10, 5, 5);
    //     for _ in 0..6 {
    //         orders_list.push(order.clone());
    //     }
    //     let orders = Arc::new(RwLock::new(orders_list));
    //     let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

    //     let coffee_maker = CoffeeMaker::new(0, 100, 50, 10);
    //     coffee_maker
    //         .clone()
    //         .start(&orders, orders_processed)
    //         .expect("Error when starting");

    //     let grain_coffee_got = coffee_maker.containers.get_quantity_of(&"grain_coffee".to_string())
    //         .expect("Error when locking coffee container");

    //     assert_eq!(grain_coffee_got, 50);
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
