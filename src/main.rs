use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread::{self, JoinHandle};

use tp1::coffee_maker::CoffeeMaker;
use tp1::errors::Error;
use tp1::input_controller::InputController;
use tp1::stats_presenter::presenter::show_statistics;

const COFFEE_MAKERS: u32 = 5;
const VALUE_TO_REPLENISH: u32 = 50;
const MIN_VALUE_TO_REPLENISH: u32 = 10;
const INITIAL_QUANTITY: u32 = 100;

fn get_coffee_makers() -> Vec<CoffeeMaker> {
    let mut coffee_makers = Vec::new();
    for j in 0..COFFEE_MAKERS {
        coffee_makers.push(CoffeeMaker::new(
            j,
            INITIAL_QUANTITY,
            VALUE_TO_REPLENISH,
            MIN_VALUE_TO_REPLENISH,
        ));
    }

    coffee_makers
}

fn main() -> Result<(), Error> {
    let icontroller = InputController::new(std::env::args().nth(1))?;
    let orders = Arc::new(RwLock::new(icontroller.get_orders()?));
    let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));
    let coffee_makers = get_coffee_makers();

    let mut machines: Vec<JoinHandle<()>> = Vec::new();
    for coffee_maker in coffee_makers.clone() {
        let orders = orders.clone();
        let orders_processed = orders_processed.clone();
        let coffee_maker_clone = coffee_maker.clone();
        let handle = thread::spawn(move || {
            match coffee_maker_clone.clone().start(&orders, orders_processed) {
                Ok(_) => println!("[COFFEE MAKER {:?}]: FINALIZING", coffee_maker.id),
                Err(err) => {
                    println!(
                        "[COFFEE MAKER {:?}]: ABORTING FOR ERROR {:?}",
                        coffee_maker.id, err
                    )
                }
            }
        });
        machines.push(handle);
    }

    show_statistics(coffee_makers, orders_processed, orders)?;

    for handle in machines {
        match handle.join() {
            Ok(_) => println!("[COFFEE MAKER]: FINISHING"),
            Err(_) => println!("[COFFEE MAKER]: ERROR WHEN JOINING"),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Condvar, Mutex, RwLock},
        thread::{self, JoinHandle},
    };

    use tp1::{coffee_maker::CoffeeMaker, orders::Order};

    #[test]
    fn test01_two_coffee_makers_with_two_dispensers_that_make_three_orders() {
        let mut orders_list = Vec::new();
        let order = Order::new(10, 10, 5, 5);
        for _ in 0..3 {
            orders_list.push(order.clone());
        }
        let orders_processed = Arc::new((Mutex::new(0), Condvar::new()));

        let mut coffee_makers = Vec::new();
        for j in 0..2 {
            coffee_makers.push(CoffeeMaker::new(j, 100, 50, 20));
        }
        let orders = Arc::new(RwLock::new(orders_list));

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

        let coffee_maker_0 = &coffee_makers[0];
        let coffee_maker_1 = &coffee_makers[1];

        let coffee_0 = coffee_maker_0
            .containers
            .get_quantity_of(&"coffee".to_string())
            .expect("Error when locking coffee container");
        let foam_0 = coffee_maker_0
            .containers
            .get_quantity_of(&"foam".to_string())
            .expect("Error when locking foam container");
        let water_0 = coffee_maker_0
            .containers
            .get_quantity_of(&"water".to_string())
            .expect("Error when locking water container");
        let cocoa_0 = coffee_maker_0
            .containers
            .get_quantity_of(&"cocoa".to_string())
            .expect("Error when locking cocoa container");

        let coffee_1 = coffee_maker_1
            .containers
            .get_quantity_of(&"coffee".to_string())
            .expect("Error when locking coffee container");
        let foam_1 = coffee_maker_1
            .containers
            .get_quantity_of(&"foam".to_string())
            .expect("Error when locking foam container");
        let water_1 = coffee_maker_1
            .containers
            .get_quantity_of(&"water".to_string())
            .expect("Error when locking water container");
        let cocoa_1 = coffee_maker_1
            .containers
            .get_quantity_of(&"cocoa".to_string())
            .expect("Error when locking cocoa container");

        assert_ne!(coffee_0, coffee_1);
        assert_ne!(water_0, water_1);
        assert_ne!(cocoa_0, cocoa_1);
        assert_ne!(foam_0, foam_1);
    }
}
