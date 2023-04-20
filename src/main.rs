use std::sync::{Arc, RwLock};

use tp1::coffee_maker::CoffeeMaker;
use tp1::errors::Error;
use tp1::input_controller::InputController;

const COFFEE_MAKER: i32 = 2;

fn main() -> Result<(), Error> {
    let icontroller = InputController::new(std::env::args().nth(1))?;
    let orders: Arc<RwLock<Vec<tp1::orders::Order>>> =
        Arc::new(RwLock::new(icontroller.get_orders()?));

    let mut coffee_makers = Vec::new();
    for j in 0..COFFEE_MAKER {
        coffee_makers.push(CoffeeMaker::new(j));
        let coffee_maker = coffee_makers[j as usize].clone();
        coffee_maker.work(&orders)?;
    }

    Ok(())
}
