use std::collections::HashMap;

use crate::{coffee_maker::CoffeeMaker, containers::Containers, orders::Order};

#[derive(Clone)]
pub struct Presentator {
    level_of_containers: Vec<HashMap<String, u32>>,
    orders_processed: u32,
    ingredients_consumed: HashMap<String, u32>,
}

const INGREDIENTS: [&str; 6] = ["COFFEE", "WATER", "COCOA", "FOAM", "GRAIN_COFFEE", "MILK"];

impl Presentator {
    pub fn new(
        self,
        coffee_makers: Vec<CoffeeMaker>,
        orders: Vec<Order>,
        initial_quantity: u32,
    ) -> Presentator {
        Presentator {
            level_of_containers: self.clone().get_info_of(coffee_makers),
            orders_processed: orders.len() as u32,
            ingredients_consumed: self.get_ingredients_consumed(initial_quantity),
        }
    }

    fn get_quantity_of(self, containers: Containers) -> HashMap<String, u32> {
        let mut level_of_containers = HashMap::new();
        for ingredient in INGREDIENTS {
            if let Ok(container) = containers.all[&ingredient.to_owned()].read() {
                level_of_containers.insert(ingredient.to_owned(), container.quantity);
            }
        }

        level_of_containers
    }

    fn get_info_of(self, coffee_makers: Vec<CoffeeMaker>) -> Vec<HashMap<String, u32>> {
        let mut vec = Vec::new();
        for coffee_maker in coffee_makers {
            vec.push(self.clone().get_quantity_of(coffee_maker.containers));
        }

        vec
    }

    fn get_ingredients_consumed(self, initial: u32) -> HashMap<String, u32> {
        let mut ingredients_consumed = HashMap::new();
        for ingredient in INGREDIENTS {
            let mut current = 0;
            for i in self.clone().level_of_containers {
                current += i[&ingredient.to_owned()];
            }
            ingredients_consumed.insert(ingredient.to_owned(), initial - current);
        }

        ingredients_consumed
    }

    // Shows the current quantity of all the containers
    pub fn present_level_of_containers(self) {
        println!("[LEVEL OF CONTAINERS]");
        for id in 0..self.level_of_containers.len() {
            for ingredient in INGREDIENTS {
                let quantity = self.level_of_containers[id][&ingredient.to_owned()];
                println!(
                    "[{:?} CONTAINER] OF [COFFEE MACHINE {:?}]: {:?}",
                    ingredient, id, quantity
                );
            }
        }
    }

    // Shows the current quantity of ingredients conssumed
    pub fn present_ingredients_consumed(self) {
        println!("[INGREDIENTS CONSSUMED]");

        for ingredient in INGREDIENTS {
            let quantity = self.ingredients_consumed[&ingredient.to_owned()];
            println!("CONSUMPTION OF {:?}: {:?}", ingredient, quantity);
        }
    }

    // Shows the stats
    pub fn present_stats(self) {
        self.clone().present_level_of_containers();
        println!("[TOTAL ORDERS PROCESSED]: {:?}", self.orders_processed);
        self.present_ingredients_consumed();
    }
}
