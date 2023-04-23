use crate::coffee_grinder::CoffeeGrinder;
use crate::containers::Containers;
use crate::{errors::Error, orders::Order};
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

const DISPENSERS: i32 = 2;

#[derive(Clone)]
pub struct CoffeeMaker {
    pub id: i32,
    pub containers: Containers,
    pub grinder: CoffeeGrinder,
}

impl CoffeeMaker {
    // Creates a coffee maker with its container of ingredients and its id
    pub fn new(id_value: i32) -> CoffeeMaker {
        let containers = Containers::new();
        CoffeeMaker {
            id: id_value,
            containers: containers.clone(),
            grinder: CoffeeGrinder::new(containers, id_value),
        }
    }

    // Gets an order from the list of orders if there are more orders to make, returns an error if not
    fn get_order(self, orders: Arc<RwLock<Vec<Order>>>, dispenser_id: i32) -> Result<Order, Error> {
        let order = if let Ok(mut orders) = orders.write() {
            if !orders.is_empty() {
                orders.remove(0)
            } else {
                println!(
                    "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: CANT HAVE ORDERS LOCK",
                    dispenser_id, self.id
                );
                return Err(Error::NoMoreOrders);
            }
        } else {
            return Err(Error::CantWriteOrdersLock);
        };

        Ok(order)
    }

    // Gets an order and processes it if it can, returns an error if not
    pub fn process_order(
        mut self,
        orders: Arc<RwLock<Vec<Order>>>,
        dispenser_id: i32,
    ) -> Result<(), Error> {
        loop {
            match self.clone().get_order(orders.clone(), dispenser_id) {
                Ok(order) => {
                    println!(
                        "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: MAKING {:?}",
                        dispenser_id, self.id, order
                    );
                    self.clone().grinder.grind_coffee()?;
                    self.containers
                        .get_ingredients(order, dispenser_id, self.id)?;
                }
                Err(error) => return Err(error),
            }
        }
    }

    // Makes the dispensers to work
    pub fn work(&self, orders: &Arc<RwLock<Vec<Order>>>) -> Result<(), Error> {
        let mut dispensers: Vec<JoinHandle<()>> = Vec::new();
        for i in 0..DISPENSERS {
            let orders = Arc::clone(orders);
            let coffee_maker = self.clone();
            let handle = thread::spawn(move || {
                println!(
                    "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: STARTING",
                    i,
                    coffee_maker.clone().id
                );
                match coffee_maker.clone().process_order(orders, i) {
                    Ok(_) => println!(
                        "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: FINALIZING",
                        i,
                        coffee_maker.clone().id
                    ),
                    Err(error) => {
                        match error {
                            Error::NotEnoughIngredient => {
                                println!("[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: THERE ARE NO MORE INGREDIENTS", i, coffee_maker.clone().id)
                            }
                            Error::NoMoreOrders => {
                                println!("[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: THERE ARE NO MORE ORDERS", i, coffee_maker.clone().id);
                            }
                            Error::CantWriteContainerLock => {
                                println!("[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: CANT HAVE CONTAINERS LOCK", i, coffee_maker.clone().id);
                            }
                            Error::CantWriteOrdersLock => {
                                println!("[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: CANT HAVE ORDERS LOCK", i, coffee_maker.clone().id);
                            }
                            _ => println!(
                                "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: ABORTING FOR {:?}",
                                i, coffee_maker.id, error
                            ),
                        }
                    }
                }
            });
            dispensers.push(handle);
        }

        for handle in dispensers {
            match handle.join() {
                Ok(_) => println!("[DISPENSER] OF [COFFEE MAKER {:?}]: FINALIZING", self.id),
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
    use std::sync::{Arc, RwLock};

    use crate::errors::Error;
    use crate::{coffee_maker::CoffeeMaker, orders::Order};

    #[test]
    fn test01_get_an_order_that_cant_be_completed() {
        let coffee_maker = CoffeeMaker::new(0);
        let order = Order::new(110, 100, 100, 100);
        let mut vec = Vec::new();
        vec.push(order);
        let orders = Arc::new(RwLock::new(vec));

        let result = coffee_maker
            .process_order(orders, 0)
            .expect_err("There is not enough ingredient to make the order");
        let err_expected = Error::NotEnoughIngredient;

        assert_eq!(result, err_expected);
    }

    #[test]
    fn test02_get_an_order_when_there_are_no_orders() {
        let coffee_maker = CoffeeMaker::new(0);
        let vec = Vec::new();
        let orders = Arc::new(RwLock::new(vec));

        let result = coffee_maker
            .process_order(orders, 0)
            .expect_err("There are no more orders");
        let err_expected = Error::NoMoreOrders;

        assert_eq!(result, err_expected);
    }

    #[test]
    fn test03_makes_one_orders_and_the_quantity_of_its_containers_get_updated() {
        let mut list_orders = Vec::new();
        let order = Order::new(10, 10, 5, 5);
        list_orders.push(order);
        let orders: Arc<RwLock<Vec<Order>>> = Arc::new(RwLock::new(list_orders));

        let coffee_maker = CoffeeMaker::new(0);
        coffee_maker.work(&orders).expect("Error when working");

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
        let mut list_orders = Vec::new();
        let order = Order::new(10, 10, 5, 5);
        for _ in 0..5 {
            list_orders.push(order.clone());
        }
        let orders: Arc<RwLock<Vec<Order>>> = Arc::new(RwLock::new(list_orders));

        let coffee_maker = CoffeeMaker::new(0);
        coffee_maker.work(&orders).expect("Error when working");

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
    fn test05_makes_ten_orders_and_the_quantity_of_its_containers_get_updated() {
        let mut list_orders = Vec::new();
        let order = Order::new(10, 10, 5, 5);
        for _ in 0..10 {
            list_orders.push(order.clone());
        }
        let orders: Arc<RwLock<Vec<Order>>> = Arc::new(RwLock::new(list_orders));

        let coffee_maker = CoffeeMaker::new(0);
        coffee_maker.work(&orders).expect("Error when working");

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
}
