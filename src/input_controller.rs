use crate::{errors::Error, orders::Order};

#[derive(Clone, Debug)]
pub struct InputController {
    pub filename: String,
}

impl InputController {
    // Creates an input controller with the filename that the user enters
    pub fn new(input: Option<String>) -> Result<InputController, Error> {
        let file = match input {
            Some(file) => file,
            None => return Err(Error::NotFileInput),
        };

        Ok(InputController { filename: file })
    }

    // Converts the orders from a json file to a vector of orders if it can, returns an error if not.
    pub fn deserialize(self, orders: &str) -> Result<Vec<Order>, Error> {
        let result = match serde_json::from_str::<Vec<Order>>(orders) {
            Ok(orders) => orders,
            Err(_) => return Err(Error::WrongFileFormat),
        };

        Ok(result)
    }

    // Reads the filename entered from user and returns a vector of orders if it can, returns an error if not.
    pub fn get_orders(self) -> Result<Vec<Order>, Error> {
        let orders = match std::fs::read_to_string(self.clone().filename) {
            Ok(orders) => orders,
            Err(_e) => return Err(Error::FileNotFound),
        };

        Ok(self.deserialize(&orders))?
    }
}

mod tests {

    #[test]
    fn test01_get_a_valid_filename() {
        let icontrolller =
            crate::input_controller::InputController::new(Some("orders.json".to_string())).unwrap();
        let expected_file = "orders.json".to_string();
        let got_file = icontrolller.filename;
        assert_eq!(expected_file, got_file);
    }

    #[test]
    fn test02_not_get_a_filename() {
        let result = crate::input_controller::InputController::new(None).unwrap_err();
        let err_expected = crate::errors::Error::NotFileInput;

        assert_eq!(result, err_expected);
    }

    #[test]
    fn test03_get_a_not_found_filename() {
        let icontroller =
            crate::input_controller::InputController::new(Some("pedidos.json".to_string()))
                .unwrap();
        let result = icontroller.get_orders().unwrap_err();
        let err_expected = crate::errors::Error::FileNotFound;

        assert_eq!(result, err_expected);
    }

    #[test]
    fn test04_get_an_order_without_all_fields() {
        let icontroller =
            crate::input_controller::InputController::new(Some("orders.json".to_string())).unwrap();
        let orders = "{\r\n    \"all\":[\r\n        {\r\n            \"water\": 10,\r\n            \"cocoa\": 2,\r\n            \"foam\": 2\r\n        }\r\n    ]\r\n}".to_string();
        let result = icontroller.deserialize(&orders).unwrap_err();
        let err_expected = crate::errors::Error::WrongFileFormat;

        assert_eq!(result, err_expected);
    }
}
