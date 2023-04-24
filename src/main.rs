use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

use tp1::coffee_maker::CoffeeMaker;
use tp1::errors::Error;
use tp1::input_controller::InputController;

const COFFEE_MAKERS: i32 = 3;

fn main() -> Result<(), Error> {
    let icontroller = InputController::new(std::env::args().nth(1))?;
    let orders: Arc<RwLock<Vec<tp1::orders::Order>>> =
        Arc::new(RwLock::new(icontroller.get_orders()?));

    let mut coffee_makers = Vec::new();
    for j in 0..COFFEE_MAKERS {
        coffee_makers.push(CoffeeMaker::new(j, 100));
    }

    let mut machines: Vec<JoinHandle<()>> = Vec::new();
    for coffee_maker in coffee_makers {
        let orders = orders.clone();
        let coffee_maker_clone = coffee_maker.clone();
        let handle = thread::spawn(move || match coffee_maker_clone.clone().work(&orders) {
            Ok(_) => println!("[COFFEE MAKER {:?}]: FINALIZING", coffee_maker.id),
            Err(err) => {
                println!(
                    "[COFFEE MAKER {:?}]: ABORTING FOR ERROR {:?}",
                    coffee_maker.id, err
                )
            }
        });
        machines.push(handle);
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
        sync::{Arc, RwLock},
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

        let orders: Arc<RwLock<Vec<Order>>> = Arc::new(RwLock::new(list_orders));
        let mut machines: Vec<JoinHandle<()>> = Vec::new();
        for coffee_maker in coffee_makers.clone() {
            let orders = orders.clone();
            let handle = thread::spawn(move || {
                let coffee_maker_clone = coffee_maker.clone();
                match coffee_maker_clone.work(&orders) {
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
