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
    fn test01_get_a_valid_filename() {
        let expected_file = "orders.json".to_string();
        let got_file =
            crate::input_controller::icontroller::get_filename(Some("orders.json".to_string()))
                .unwrap();
        assert_eq!(expected_file, got_file);
    }

    #[test]
    fn test02_not_get_a_filename() {
        let result = crate::input_controller::icontroller::get_filename(None).unwrap_err();
        let err_expected = crate::errors::Error::NotFileInput;

        assert_eq!(result, err_expected);
    }

    #[test]
    fn test03_get_a_not_found_filename() {
        let result = crate::input_controller::icontroller::open_file(&"pedidos.json".to_string())
            .unwrap_err();
        let err_expected = crate::errors::Error::FileNotFound;

        assert_eq!(result, err_expected);
    }

    #[test]
    fn test04_get_an_order_without_all_fields() {
        let orders = "{\r\n    \"all\":[\r\n        {\r\n            \"water\": 10,\r\n            \"cocoa\": 2,\r\n            \"foam\": 2\r\n        }\r\n    ]\r\n}".to_string();
        let result = crate::input_controller::icontroller::deserialize(&orders).unwrap_err();
        let err_expected = crate::errors::Error::WrongFileFormat;

        assert_eq!(result, err_expected);
    }
}
