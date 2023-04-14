use tp1::{
    errors::Error,
    input_controller::icontroller::{deserialize, get_file_name, open_file},
};

fn main() -> Result<(), Error> {
    let file = get_file_name()?;

    let orders = {
        let orders = open_file(&file)?;
        deserialize(&orders)?
    };

    for idx in 0..orders.all.len() {
        println!("{:?}", orders.all[idx]);
    }

    Ok(())
}
