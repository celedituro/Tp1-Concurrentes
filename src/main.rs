use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use tp1::coffee_maker::CoffeeMaker;
use tp1::errors::Error;
use tp1::input_controller::InputController;
use tp1::orders::Order;
use tp1::stats_presenter::presenter::present_stats;

const COFFEE_MAKERS: u32 = 2;
const INITIAL_QUANTITY: u32 = 100;

fn show_stats(coffee_makers: Vec<CoffeeMaker>, orders_processed: Arc<(Mutex<i32>, Condvar)>) {
    let (orders_processed_lock, condvar) = &*orders_processed;
    if let Ok(orders_processed) = orders_processed_lock.lock() {
        if let Ok(orders_processed) = condvar.wait_while(orders_processed, |num| *num == 0) {
            println!("PRESENTING STATS WITH NUM ORDERS: {:?}", orders_processed);
            present_stats(
                coffee_makers.clone(),
                *orders_processed as u32,
                INITIAL_QUANTITY * coffee_makers.len() as u32,
            );
        }
    }
}

fn present_statistics(
    coffee_makers: Vec<CoffeeMaker>,
    orders_processed: Arc<(Mutex<i32>, Condvar)>,
    orders: Arc<RwLock<Vec<Order>>>,
) {
    let presenter_handle = thread::spawn(move || loop {
        println!("[PRESENTER]: MAKING STATS");
        show_stats(coffee_makers.clone(), orders_processed.clone());
        if let Ok(orders) = orders.read() {
            if orders.is_empty() {
                println!("[PRESENTER]: NO MORE ORDERS");
                break;
            }
        }
        thread::sleep(Duration::from_secs(1));
    });

    match presenter_handle.join() {
        Ok(_) => println!("[PRESENTER]: FINALIZING"),
        Err(_) => println!("[PRESENTER]: ERROR WHEN JOINING"),
    };
}

fn get_coffee_makers() -> Vec<CoffeeMaker> {
    let mut coffee_makers = Vec::new();
    for j in 0..COFFEE_MAKERS {
        coffee_makers.push(CoffeeMaker::new(j, INITIAL_QUANTITY));
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

    present_statistics(coffee_makers, orders_processed, orders);

    for handle in machines {
        match handle.join() {
            Ok(_) => println!("[COFFEE MAKER]: FINALIZING"),
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
            coffee_makers.push(CoffeeMaker::new(j, 100));
        }
        let orders = Arc::new(RwLock::new(orders_list));

        let mut machines: Vec<JoinHandle<()>> = Vec::new();
        for coffee_maker in coffee_makers.clone() {
            let orders = orders.clone();
            let orders_processed = orders_processed.clone();
            let handle = thread::spawn(move || {
                let coffee_maker_clone = coffee_maker.clone();
                match coffee_maker_clone.start(&orders, orders_processed) {
                    Ok(_) => println!("[COFFEE MAKER {:?}]: FINALIZING", coffee_maker.id),
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

        let coffee_0 = coffee_maker_0.clone().containers.all["coffee"]
            .read()
            .expect("Coffee maker 0 cant have read lock of the coffee container")
            .quantity;
        let coffee_1 = coffee_maker_1.clone().containers.all["coffee"]
            .read()
            .expect("Coffee maker 1 cant have read lock of the coffee container")
            .quantity;
        assert_ne!(coffee_0, coffee_1);
        let water_0 = coffee_maker_0.clone().containers.all["water"]
            .read()
            .expect("Coffee maker 0 cant have read lock of the water container")
            .quantity;
        let water_1 = coffee_maker_1.clone().containers.all["water"]
            .read()
            .expect("Coffee maker 1 cant have read lock of the water container")
            .quantity;
        assert_ne!(water_0, water_1);
        let cocoa_0 = coffee_maker_0.clone().containers.all["cocoa"]
            .read()
            .expect("Coffee maker 0 cant have read lock of the cocoa container")
            .quantity;
        let cocoa_1 = coffee_maker_1.clone().containers.all["cocoa"]
            .read()
            .expect("Coffee maker 1 cant have read lock of the cocoa container")
            .quantity;
        assert_ne!(cocoa_0, cocoa_1);
        let foam_0 = coffee_maker_0.clone().containers.all["foam"]
            .read()
            .expect("Coffee maker 0 cant have read lock of the foam container")
            .quantity;
        let foam_1 = coffee_maker_1.clone().containers.all["foam"]
            .read()
            .expect("Coffee maker 1 cant have read lock of the foam container")
            .quantity;
        assert_ne!(foam_0, foam_1);
    }
}
