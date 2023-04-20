use std::sync::{Arc, RwLock};
use std::thread;

use tp1::coffee_maker::CoffeeMaker;
use tp1::errors::Error;
use tp1::input_controller::InputController;

const DISPENSERS: i32 = 3;
const COFFEE_MAKER: i32 = 2;

fn main() -> Result<(), Error> {
    let icontroller = InputController::new(std::env::args().nth(1))?;
    let orders: Arc<RwLock<Vec<tp1::orders::Order>>> =
        Arc::new(RwLock::new(icontroller.get_orders()?));

    let mut coffee_makers = Vec::new();
    for j in 0..COFFEE_MAKER {
        coffee_makers.push(CoffeeMaker::new(j));
        let mut dispensers: Vec<thread::JoinHandle<()>> = Vec::new();
        for i in 0..DISPENSERS {
            let orders = Arc::clone(&orders);
            let coffee_maker = coffee_makers[j as usize].clone();
            let handle = thread::spawn(move || {
                println!("[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: STARTING", i, j);
                match coffee_maker.process_order(orders, i) {
                    Ok(_) => println!("[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: FINALIZING", i, j),
                    Err(error) => {
                        match error {
                            Error::NotEnoughIngredient => {
                                println!("[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: THERE ARE NO MORE INGREDIENTS", i, j)
                            }
                            Error::NoMoreOrders => {
                                println!("[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: THERE ARE NO MORE ORDERS", i, j);
                            }
                            _ => println!(
                                "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: ABORTING FOR {:?}",
                                i, j, error
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
    }

    Ok(())
}
