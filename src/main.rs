use std::sync::{Arc, RwLock};
use std::thread;

use tp1::coffee_maker::coffe_maker::get_order;
use tp1::containers::Containers;
use tp1::errors::Error;
use tp1::input_controller::InputController;

const DISPENSERS: i32 = 3;

fn main() -> Result<(), Error> {
    let icontroller = InputController::new(std::env::args().nth(1))?;
    let orders = Arc::new(RwLock::new(icontroller.get_orders()?));
    let containers = Arc::new(RwLock::new(Containers::new()));

    let mut dispensers = Vec::new();

    for i in 0..DISPENSERS {
        let container = Arc::clone(&containers);
        let orders = Arc::clone(&orders);

        let handle = thread::spawn(move || {
            println!("[DISPENSER {:?}] STARTING", i);
            match get_order(orders, container, i) {
                Ok(_) => println!("[DISPENSER {:?}] FINALIZING", i),
                Err(e) => println!("[DISPENSER {:?}] ABORTING FOR {:?}", i, e),
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
