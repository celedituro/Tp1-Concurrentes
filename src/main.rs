use std::sync::{Arc, RwLock};
use std::thread;

use tp1::coffee_maker::coffe_maker::get_order;
use tp1::containers::Containers;
use tp1::errors::Error;
use tp1::input_controller::InputController;

const DISPENSERS: i32 = 10;

fn main() -> Result<(), Error> {
    let icontroller = InputController::new(std::env::args().nth(1))?;
    let orders: Arc<RwLock<Vec<tp1::orders::Order>>> =
        Arc::new(RwLock::new(icontroller.get_orders()?));
    let containers = Containers::new();

    let mut dispensers: Vec<thread::JoinHandle<()>> = Vec::new();

    for i in 0..DISPENSERS {
        let container = containers.clone();
        let orders = Arc::clone(&orders);

        let handle = thread::spawn(move || {
            println!("[DISPENSER {:?}] STARTING", i);
            match get_order(orders, container, i) {
                Ok(_) => println!("[DISPENSER {:?}] FINALIZING", i),
                Err(error) => match error {
                    Error::NotEnoughIngredient => {
                        println!("[DISPENSER {:?}] THERE ARE NO MORE INGREDIENTS", i)
                    }
                    Error::NoMoreOrders => {
                        println!("[DISPENSER {:?}] THERE ARE NO MORE ORDERS", i);
                    }
                    _ => println!("[DISPENSER {:?}] ABORTING FOR {:?}", i, error),
                },
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
