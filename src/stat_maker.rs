pub mod stats_maker {

    use std::collections::HashMap;

    use crate::{coffee_maker::CoffeeMaker, containers::Containers};

    const INGREDIENTS: [&str; 6] = ["coffee", "water", "cocoa", "foam", "grain_coffee", "milk"];

    /// Gets the current quantity of all the containers of all the coffee machines.
    fn get_quantity_of(containers: Containers) -> HashMap<String, u32> {
        let mut level_of_containers = HashMap::new();
        for ingredient in INGREDIENTS {
            if let Ok(quantity) = containers.clone().get_quantity_of(&ingredient.to_owned()) {
                level_of_containers.insert(ingredient.to_owned(), quantity);
            }
        }

        level_of_containers
    }

    /// Returns a list of hashmaps. Every element of the list represents a coffee machine
    /// and every hashmap represents its container where the key is the ingredient of it and
    /// the value is its quantity.
    pub fn get_containers_info(coffee_makers: Vec<CoffeeMaker>) -> Vec<HashMap<String, u32>> {
        let mut vec = Vec::new();
        for coffee_maker in coffee_makers {
            vec.push(get_quantity_of(coffee_maker.containers));
        }

        vec
    }

    /// Shows the current quantity of ingredients consumed between all the containers of all the
    /// coffee machines.
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
}
