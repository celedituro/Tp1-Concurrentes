use tp1::coffee_maker::CoffeeMaker;
use tp1::containers::Containers;
use tp1::{
    errors::Error,
    input_controller::icontroller::{deserialize, get_filename, open_file},
};

fn main() -> Result<(), Error> {
    let file = get_filename(std::env::args().nth(1))?;

    let orders = {
        let orders = open_file(&file)?;
        deserialize(&orders)?
    };

    let containers = Containers::new();
    let mut coffee_maker = CoffeeMaker::new(1);

    for idx in 0..orders.all.len() {
        let current_order = &orders.all[idx];
        println!("{:?}", current_order);

        coffee_maker.get_order(containers.clone(), current_order)?;
    }

    println!("{:?}", containers);
    Ok(())
}
