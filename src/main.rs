use tp1::coffee_maker::CoffeeMaker;
use tp1::containers::Containers;
use tp1::errors::Error;
use tp1::input_controller::InputController;

fn main() -> Result<(), Error> {
    let icontroller = InputController::new(std::env::args().nth(1))?;
    let orders = icontroller.get_orders()?;

    let containers = Containers::new();
    let mut coffee_maker = CoffeeMaker::new(1);

    for order in &orders {
        println!("{:?}", order);
        coffee_maker.get_order(containers.clone(), order)?;
    }

    Ok(())
}
