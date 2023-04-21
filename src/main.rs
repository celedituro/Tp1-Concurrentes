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
        coffee_makers.push(CoffeeMaker::new(j));
    }

    let mut machines: Vec<JoinHandle<()>> = Vec::new();
    for coffee_maker in coffee_makers {
        let orders = orders.clone();
        let handle = thread::spawn(move || {
            let coffee_maker = coffee_maker.clone();
            match coffee_maker.work(&orders) {
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

    for handle in machines {
        match handle.join() {
            Ok(_) => println!("[COFFEE MAKER]: FINALIZING"),
            Err(_) => println!("[COFFEE MAKER]: ERROR WHEN JOINING"),
        }
    }

    Ok(())
}
