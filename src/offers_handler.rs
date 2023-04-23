pub mod offer_handler {
    use crate::{
        coffee_maker::CoffeeMaker, dispensers::dispenser::get_ingredients, errors::Error,
        ingredient_handler::IngredientHandler, orders::Order,
    };
    use std::sync::{Arc, RwLock};

    const COFFEE: &str = "coffee";
    //const FOAM: &str = "foam";

    // Gets an order from the list of orders if there are more orders to make, returns an error if not
    fn get_order(
        orders: Arc<RwLock<Vec<Order>>>,
        dispenser_id: i32,
        coffee_maker_id: i32,
    ) -> Result<Order, Error> {
        let order = if let Ok(mut orders) = orders.write() {
            if !orders.is_empty() {
                orders.remove(0)
            } else {
                println!(
                    "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: CANT HAVE ORDERS LOCK",
                    dispenser_id, coffee_maker_id
                );
                return Err(Error::NoMoreOrders);
            }
        } else {
            return Err(Error::CantWriteOrdersLock);
        };

        Ok(order)
    }

    pub fn get_more_ingredients(
        mut handler: IngredientHandler,
        dispenser_id: i32,
    ) -> Result<(), Error> {
        println!("[DISPENSER {:?}]: GETTING MORE INGREDIENTS", dispenser_id);
        handler.get_more_ingredient(COFFEE.to_owned(), dispenser_id)?;
        //handler.get_more_ingredient(FOAM.to_owned(), dispenser_id)?;

        Ok(())
    }

    // Gets an order and processes it if it can, returns an error if not
    pub fn process_order(
        orders: Arc<RwLock<Vec<Order>>>,
        coffee_maker: CoffeeMaker,
        dispenser_id: i32,
    ) -> Result<(), Error> {
        loop {
            match get_order(orders.clone(), dispenser_id, coffee_maker.id) {
                Ok(order) => {
                    println!(
                        "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: MAKING {:?}",
                        dispenser_id, coffee_maker.id, order
                    );
                    get_more_ingredients(coffee_maker.handler.clone(), dispenser_id)?;
                    get_ingredients(
                        order,
                        coffee_maker.containers.clone(),
                        dispenser_id,
                        coffee_maker.id,
                    )?;
                }
                Err(error) => return Err(error),
            }
        }
    }
}
