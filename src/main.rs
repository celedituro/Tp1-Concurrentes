use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

use tp1::coffee_maker::CoffeeMaker;
use tp1::errors::Error;
use tp1::input_controller::InputController;
use tp1::stats_presentator::presenter::present_stats;

const COFFEE_MAKERS: u32 = 2;
const INITIAL_QUANTITY: u32 = 100;

fn main() -> Result<(), Error> {
    let icontroller = InputController::new(std::env::args().nth(1))?;
    let orders_list = icontroller.get_orders()?;
    let orders = Arc::new(Mutex::new(orders_list.clone()));
    let condvar = Arc::new(Condvar::new());
    let total_num_orders = orders_list.len() as u32;

    let mut coffee_makers = Vec::new();
    for j in 0..COFFEE_MAKERS {
        coffee_makers.push(CoffeeMaker::new(j, INITIAL_QUANTITY));
    }

    let mut machines: Vec<JoinHandle<()>> = Vec::new();
    for coffee_maker in coffee_makers.clone() {
        let orders = orders.clone();
        let condvar = condvar.clone();
        let coffee_maker_clone = coffee_maker.clone();
        let handle =
            thread::spawn(
                move || match coffee_maker_clone.clone().work(&orders, condvar) {
                    Ok(_) => println!("[COFFEE MAKER {:?}]: FINALIZING", coffee_maker.id),
                    Err(err) => {
                        println!(
                            "[COFFEE MAKER {:?}]: ABORTING FOR ERROR {:?}",
                            coffee_maker.id, err
                        )
                    }
                },
            );
        machines.push(handle);
    }

    if let Ok(mut orders) = orders.lock() {
        println!("PRESENTING STATS WITH {:?}", orders.len() as u32);
        println!("TOTAL ORDERS afuera {:?}", total_num_orders);
        while (orders.len() as u32) > 4 {
            println!("TOTAL ORDERS adentro {:?}", orders.len());
            orders = condvar.wait(orders).unwrap();
        }
        present_stats(
            coffee_makers.clone(),
            total_num_orders,
            orders.len() as u32,
            INITIAL_QUANTITY * coffee_makers.len() as u32,
        );
    }

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
        sync::{Arc, Condvar, Mutex},
        thread::{self, JoinHandle},
    };

    use tp1::{coffee_maker::CoffeeMaker, orders::Order};

    #[test]
    fn test01_two_coffee_makers_with_two_dispensers_that_make_three_orders() {
        let mut list_orders = Vec::new();
        let order = Order::new(10, 10, 5, 5);
        for _ in 0..3 {
            list_orders.push(order.clone());
        }

        let mut coffee_makers = Vec::new();
        for j in 0..2 {
            coffee_makers.push(CoffeeMaker::new(j, 100));
        }

        let orders: Arc<Mutex<Vec<Order>>> = Arc::new(Mutex::new(list_orders));
        let mut machines: Vec<JoinHandle<()>> = Vec::new();
        for coffee_maker in coffee_makers.clone() {
            let orders = orders.clone();
            let handle = thread::spawn(move || {
                let coffee_maker_clone = coffee_maker.clone();
                match coffee_maker_clone.work(&orders, Arc::new(Condvar::new())) {
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
