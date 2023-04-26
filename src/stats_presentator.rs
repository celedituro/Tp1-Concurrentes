pub mod presenter {

    use std::collections::HashMap;

    use crate::{coffee_maker::CoffeeMaker, containers::Containers};

    const INGREDIENTS: [&str; 6] = ["coffee", "water", "cocoa", "foam", "grain_coffee", "milk"];

    /// Gets the current quantity of all the containers of all the coffee machines
    pub fn get_quantity_of(containers: Containers) -> HashMap<String, u32> {
        let mut level_of_containers = HashMap::new();
        for ingredient in INGREDIENTS {
            if let Ok(container) = containers.all[&ingredient.to_owned()].read() {
                level_of_containers.insert(ingredient.to_owned(), container.quantity);
            }
        }

        level_of_containers
    }

    pub fn get_info_of(coffee_makers: Vec<CoffeeMaker>) -> Vec<HashMap<String, u32>> {
        let mut vec = Vec::new();
        for coffee_maker in coffee_makers {
            vec.push(get_quantity_of(coffee_maker.containers));
        }

        vec
    }

    /// Shows the total of orders already processed by the dispensers of all the coffee machines
    pub fn get_orders_processed(total: u32, current: u32) -> u32 {
        total - current
    }

    /// Shows the current quantity of ingredients consumed between all the containers of all the
    /// coffee machines
    pub fn get_ingredients_consumed(
        containers_level: Vec<HashMap<String, u32>>,
        initial_quantity: u32,
    ) -> HashMap<String, u32> {
        let mut ingredients_consumed = HashMap::new();
        for ingredient in INGREDIENTS {
            let mut current = 0;
            for i in containers_level.clone() {
                current += i[&ingredient.to_owned()];
            }
            ingredients_consumed.insert(ingredient.to_owned(), initial_quantity - current);
        }

        ingredients_consumed
    }

    /// Shows the current quantity of all the containers of all the coffee machines
    pub fn present_level_of_containers(containers_level: Vec<HashMap<String, u32>>) {
        println!("[LEVEL OF CONTAINERS]");
        for (id, containers) in containers_level.iter().enumerate() {
            for ingredient in INGREDIENTS {
                let quantity = containers[&ingredient.to_owned()];
                println!(
                    "[{:?} CONTAINER] OF [COFFEE MACHINE {:?}]: {:?}",
                    ingredient, id as i32, quantity
                );
            }
        }
    }

    /// Shows the current quantity of ingredients consumed between all the containers of all the
    /// coffee machines
    pub fn present_ingredients_consumed(ingredients_consumed: HashMap<String, u32>) {
        println!("[INGREDIENTS CONSSUMED]");

        for ingredient in INGREDIENTS {
            let quantity = ingredients_consumed[&ingredient.to_owned()];
            println!("CONSUMPTION OF {:?}: {:?}", ingredient, quantity);
        }
    }

    /// Shows stats of the level of containers of all the coffee machines, the total of orders processed
    /// and the total of ingredients consumed between all the containers of all the coffee machines
    pub fn present_stats(
        coffee_makers: Vec<CoffeeMaker>,
        total_num_orders: u32,
        current_num_orders: u32,
        initial_quantity: u32,
    ) {
        let containers_level = get_info_of(coffee_makers);
        present_level_of_containers(containers_level.clone());
        println!(
            "[TOTAL ORDERS PROCESSED]: {:?}",
            get_orders_processed(total_num_orders, current_num_orders)
        );
        let ingredients_consumed = get_ingredients_consumed(containers_level, initial_quantity);
        present_ingredients_consumed(ingredients_consumed);
    }
}
