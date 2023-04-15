pub mod icontroller {
    use crate::{errors::Error, orders::Orders};

    // Gets the filename of orders if it can, returns an error if not.
    pub fn get_filename(input: Option<String>) -> Result<String, Error> {
        let file = match input {
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

mod tests {

    #[test]
    fn get_a_valid_filename() {
        let expected_file = "orders.json".to_string();
        let got_file =
            crate::input_controller::icontroller::get_filename(Some("orders.json".to_string()))
                .unwrap();
        assert_eq!(expected_file, got_file);
    }
}
