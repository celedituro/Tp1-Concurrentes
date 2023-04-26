use crate::containers::Containers;
use crate::ingredient_handler::IHandler;
use crate::orders_handler::order_handler::process_order;
use crate::{errors::Error, orders::Order};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread::{self, JoinHandle};

const DISPENSERS: u32 = 4;
const MIN_VALUE_TO_REPLENISH: u32 = 20;
const REPLENISH_VALUE: u32 = 50;

#[derive(Clone)]
pub struct CoffeeMaker {
    pub id: u32,
    pub containers: Containers,
    pub handler: IHandler,
}

impl CoffeeMaker {
    // Creates a coffee maker with its container of ingredients and its id
    pub fn new(id_value: u32, initial_quantity: u32) -> CoffeeMaker {
        let containers = Containers::new(initial_quantity);
        CoffeeMaker {
            id: id_value,
            containers: containers.clone(),
            handler: IHandler::new(
                containers,
                id_value,
                MIN_VALUE_TO_REPLENISH,
                REPLENISH_VALUE,
            ),
        }
    }

    // Makes the dispensers to work
    pub fn work(
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
                    i,
                    coffee_machine.clone().id
                );
                match process_order(orders, coffee_machine.clone(), i, orders_processed) {
                    Ok(_) => println!(
                        "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: FINALIZING",
                        i,
                        coffee_machine.clone().id
                    ),
                    Err(error) => {
                        match error {
                            Error::NotEnoughIngredient => {
                                println!("[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: THERE ARE NO MORE INGREDIENTS", i, coffee_machine.clone().id)
                            }
                            Error::NoMoreOrders => {
                                println!("[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: THERE ARE NO MORE ORDERS", i, coffee_machine.clone().id);
                            }
                            Error::CantWriteContainerLock => {
                                println!("[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: CANT HAVE CONTAINERS LOCK", i, coffee_machine.clone().id);
                            }
                            Error::CantWriteOrdersLock => {
                                println!("[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: CANT HAVE ORDERS LOCK", i, coffee_machine.clone().id);
                            }
                            _ => println!(
                                "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: ABORTING FOR {:?}",
                                i, coffee_machine.id, error
                            ),
                        }
                    }
                }
            });
            dispensers.push(handle);
        }

        for handle in dispensers {
            match handle.join() {
                Ok(_) => println!(
                    "[DISPENSER] OF [COFFEE MAKER {:?}]: FINALIZING",
                    self.clone().id
                ),
                Err(_) => println!(
                    "[DISPENSER] OF [COFFEE MAKER {:?}]: ERROR WHEN JOINING",
                    self.clone().id
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
        let coffee_maker = CoffeeMaker::new(0, 100);
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
        let coffee_maker = CoffeeMaker::new(0, 100);
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

        let coffee_maker = CoffeeMaker::new(0, 100);
        coffee_maker
            .clone()
            .work(&orders, orders_processed)
            .expect("Error when working");

        let coffee = coffee_maker.clone().containers.all["coffee"]
            .read()
            .expect("Cant have read lock of the coffee container")
            .quantity;
        assert_eq!(coffee, 90);
        let water = coffee_maker.clone().containers.all["water"]
            .read()
            .expect("Cant have read lock of the water container")
            .quantity;
        assert_eq!(water, 90);
        let cocoa = coffee_maker.clone().containers.all["cocoa"]
            .read()
            .expect("Cant have read lock of the cocoa container")
            .quantity;
        assert_eq!(cocoa, 95);
        let foam = coffee_maker.containers.all["foam"]
            .read()
            .expect("Cant have read lock of foam coffee container")
            .quantity;
        assert_eq!(foam, 95);
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

        let coffee_maker = CoffeeMaker::new(0, 100);
        coffee_maker
            .clone()
            .work(&orders, orders_processed)
            .expect("Error when working");

        let coffee = coffee_maker.clone().containers.all["coffee"]
            .read()
            .expect("Cant have read lock of the coffee container")
            .quantity;
        assert_eq!(coffee, 50);
        let water = coffee_maker.clone().containers.all["water"]
            .read()
            .expect("Cant have read lock of the water container")
            .quantity;
        assert_eq!(water, 50);
        let cocoa = coffee_maker.clone().containers.all["cocoa"]
            .read()
            .expect("Cant have read lock of the cocoa container")
            .quantity;
        assert_eq!(cocoa, 75);
        let foam = coffee_maker.containers.all["foam"]
            .read()
            .expect("Cant have read lock of foam coffee container")
            .quantity;
        assert_eq!(foam, 75);
    }

    #[test]
    fn test05_get_more_coffee_and_decrease_quantity_of_grain_coffee_container() {
        let mut orders_list = Vec::new();
        let order = Order::new(10, 10, 5, 5);
        for _ in 0..10 {
            orders_list.push(order.clone());
        }
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let coffee_maker = CoffeeMaker::new(0, 100);
        coffee_maker
            .clone()
            .work(&orders, orders_processed)
            .expect("Error when working");

        let grain_coffee = coffee_maker.clone().containers.all["grain_coffee"]
            .read()
            .expect("Cant have read lock of the grains of coffee container")
            .quantity;
        assert_eq!(grain_coffee, 50);
        let coffee = coffee_maker.clone().containers.all["coffee"]
            .read()
            .expect("Cant have read lock of the coffee container")
            .quantity;
        assert_eq!(coffee, 50);
    }

    #[test]
    fn test06_get_more_foam_and_decrease_quantity_of_milk_container() {
        let mut orders_list = Vec::new();
        let order = Order::new(5, 10, 5, 10);
        for _ in 0..10 {
            orders_list.push(order.clone());
        }
        let orders = Arc::new(RwLock::new(orders_list));
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let coffee_maker = CoffeeMaker::new(0, 100);
        coffee_maker
            .clone()
            .work(&orders, orders_processed)
            .expect("Error when working");

        let milk = coffee_maker.clone().containers.all["milk"]
            .read()
            .expect("Cant have read lock of the milk container")
            .quantity;
        assert_eq!(milk, 50);
        let foam: u32 = coffee_maker.clone().containers.all["foam"]
            .read()
            .expect("Cant have read lock of the foam container")
            .quantity;
        assert_eq!(foam, 50);
    }
}
