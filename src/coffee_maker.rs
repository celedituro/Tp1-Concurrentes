use crate::containers::Containers;
use crate::orders_handler::order_handler::process_order;
use crate::{errors::Error, orders::Order};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread::{self, JoinHandle};

const DISPENSERS: u32 = 4;

#[derive(Clone)]
pub struct CoffeeMaker {
    pub id: u32,
    pub containers: Containers,
}

impl CoffeeMaker {
    /// Creates a new [`CoffeeMaker`].
    pub fn new(id_value: u32, initial_quantity: u32, _replenish_value: u32) -> CoffeeMaker {
        CoffeeMaker {
            id: id_value,
            containers: Containers::new(initial_quantity),
        }
    }

    fn handle_order(
        self,
        orders: Arc<RwLock<Vec<Order>>>,
        dispenser_id: u32,
        orders_processed: Arc<(Mutex<i32>, Condvar)>,
    ) {
        match process_order(orders, self.clone(), dispenser_id, orders_processed) {
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

    // Makes the dispensers to start making orders
    pub fn start(
        self,
        orders: &Arc<RwLock<Vec<Order>>>,
        orders_processed: Arc<(Mutex<i32>, Condvar)>,
    ) -> Result<(), Error> {
        let mut dispensers: Vec<JoinHandle<()>> = Vec::new();

        for i in 0..DISPENSERS {
            let orders = Arc::clone(orders);
            let orders_processed = orders_processed.clone();
            let coffee_machine = self.clone();
            let handle = thread::spawn(move || {
                println!(
                    "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: STARTING",
                    i, self.id
                );
                coffee_machine
                    .clone()
                    .handle_order(orders.clone(), i, orders_processed);
            });
            dispensers.push(handle);
        }

        for handle in dispensers {
            match handle.join() {
                Ok(_) => println!(
                    "[DISPENSER] OF [COFFEE MAKER {:?}]: FINISHING",
                    self.clone().id
                ),
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
        let coffee_maker = CoffeeMaker::new(0, 100, 50);
        let order = Order::new(110, 100, 100, 100);
        let mut orders_list = Vec::new();
        orders_list.push(order);
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let result = process_order(orders, coffee_maker, 0, orders_processed)
            .expect_err("There is not enough ingredient to make the order");
        let err_expected = Error::NotEnoughIngredient;

        assert_eq!(result, err_expected);
    }

    #[test]
    fn test02_get_an_order_when_there_are_no_orders() {
        let coffee_maker = CoffeeMaker::new(0, 100, 50);
        let orders_list = Vec::new();
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let result = process_order(orders, coffee_maker, 0, orders_processed)
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
    //     let order = Order::new(10, 10, 5, 5);
    //     for _ in 0..10 {
    //         orders_list.push(order.clone());
    //     }
    //     let orders = Arc::new(RwLock::new(orders_list));
    //     let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

    //     let coffee_maker = CoffeeMaker::new(0, 100, 50);
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
