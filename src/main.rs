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

    for idx in 0..orders.all.len() {
        println!("{:?}", orders.all[idx]);
    }

    Ok(())
}
