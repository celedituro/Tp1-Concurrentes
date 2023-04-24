pub mod dispenser {
    use crate::{containers::Containers, errors::Error, orders::Order};

    const COFFEE: &str = "coffee";
    const WATER: &str = "water";
    const COCOA: &str = "cocoa";
    const FOAM: &str = "foam";

    pub fn make_order(
        order: Order,
        mut containers: Containers,
        dispenser_id: i32,
        coffee_maker_id: i32,
    ) -> Result<(), Error> {
        containers.get_ingredient(
            &COFFEE.to_owned(),
            order.coffee,
            dispenser_id,
            coffee_maker_id,
            false,
        )?;
        containers.get_ingredient(
            &WATER.to_owned(),
            order.water,
            dispenser_id,
            coffee_maker_id,
            false,
        )?;
        containers.get_ingredient(
            &COCOA.to_owned(),
            order.cocoa,
            dispenser_id,
            coffee_maker_id,
            false,
        )?;
        containers.get_ingredient(
            &FOAM.to_owned(),
            order.foam,
            dispenser_id,
            coffee_maker_id,
            false,
        )?;

        println!(
            "[DISPENSER {:?}] OF [COFFEE MAKER {:?}]: GOT ALL INGREDIENTS",
            dispenser_id, coffee_maker_id
        );

        Ok(())
    }
}
