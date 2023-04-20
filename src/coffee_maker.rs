use crate::containers::Containers;
use crate::{errors::Error, orders::Order};
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

const DISPENSERS: i32 = 3;

#[derive(Clone)]
pub struct CoffeeMaker {
    id: i32,
    containers: Containers,
}

impl CoffeeMaker {
    // Creates a coffee maker with its container of ingredients and its id
    pub fn new(id_value: i32) -> CoffeeMaker {
        CoffeeMaker {
            id: id_value,
            containers: Containers::new(),
        }
    }

    // Gets an order from the list of orders if there are more orders to make, returns an error if not
    fn get_order(self, orders: Arc<RwLock<Vec<Order>>>) -> Result<Order, Error> {
        let order = if let Ok(mut orders) = orders.write() {
            if !orders.is_empty() {
                orders.remove(0)
            } else {
                return Err(Error::NoMoreOrders);
            }
        } else {
            return Err(Error::NoMoreOrders);
        };

        Ok(order)
    }

    // Gets an order and processes it if it can, returns an error if not
    pub fn process_order(
        mut self,
        orders: Arc<RwLock<Vec<Order>>>,
        dispenser_id: i32,
    ) -> Result<(), Error> {
        if let Ok(order) = self.clone().get_order(orders) {
            println!(
                "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: MAKING {:?}",
                dispenser_id, self.id, order
            );
            self.containers
                .get_ingredients(order, dispenser_id, self.id)?;
        } else {
            return Err(Error::NoMoreOrders);
        }

        Ok(())
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
                Ok(_) => println!("FINALIZING"),
                Err(_) => println!("ERROR WHEN JOINING"),
            }
        }

        Ok(())
    }
}

mod tests {

    #[test]
    fn test01_get_an_order_that_cant_be_completed() {
        let coffee_maker = crate::coffee_maker::CoffeeMaker::new(1);
        let order = crate::orders::Order::new(110, 100, 100, 100);
        let mut vec = Vec::new();
        vec.push(order);
        let orders = std::sync::Arc::new(std::sync::RwLock::new(vec));

        let result = coffee_maker.process_order(orders, 0).unwrap_err();
        let err_expected = crate::errors::Error::NotEnoughIngredient;

        assert_eq!(result, err_expected);
    }

    #[test]
    fn test02_get_an_order_when_there_are_no_orders() {
        let coffee_maker = crate::coffee_maker::CoffeeMaker::new(1);
        let vec = Vec::new();
        let orders = std::sync::Arc::new(std::sync::RwLock::new(vec));

        let result = coffee_maker.process_order(orders, 0).unwrap_err();
        let err_expected = crate::errors::Error::NoMoreOrders;

        assert_eq!(result, err_expected);
    }
}
