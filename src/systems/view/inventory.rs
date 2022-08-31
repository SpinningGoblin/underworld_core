use crate::components::{Inventory, InventoryView};

pub fn view(inventory: &Inventory, knows_packed: bool, knows_all: bool) -> InventoryView {
    let equipped_items = inventory
        .equipment
        .iter()
        .filter(|character_item| character_item.is_at_the_ready())
        .map(|character_item| super::character_item::view(character_item, true));

    let packed_items = inventory
        .equipment
        .iter()
        .filter(|character_item| character_item.is_packed())
        .filter_map(|character_item| {
            if knows_packed || knows_all {
                Some(super::character_item::view(character_item, true))
            } else {
                None
            }
        });

    InventoryView {
        equipment: equipped_items.chain(packed_items).collect(),
    }
}
