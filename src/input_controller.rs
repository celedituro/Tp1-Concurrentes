pub mod icontroller {
    use crate::{errors::Error, orders::Orders};

    // Gets the filename of orders if it can, returns an error if not.
    pub fn get_file_name() -> Result<String, Error> {
        let file = match std::env::args().nth(1) {
            Some(file) => file,
            None => return Err(Error::NotFileInput),
        };

        Ok(file)
    }

    // Opens the file of orders if it can, returns an error if not.
    pub fn open_file(file: &String) -> Result<String, Error> {
        let orders = match std::fs::read_to_string(file) {
            Ok(orders) => orders,
            Err(_e) => return Err(Error::FileNotFound),
        };

        Ok(orders)
    }

    // Converts the orders from a json file to objects if it can, returns an error if not.
    pub fn deserialize(orders: &str) -> Result<Orders, Error> {
        let orders = match serde_json::from_str::<Orders>(orders) {
            Ok(orders) => orders,
            Err(_) => return Err(Error::WrongFileFormat),
        };

        Ok(orders)
    }
}
