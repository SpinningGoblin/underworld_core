use crate::components::inventory::{Inventory, InventoryView};

pub fn look_at(
    inventory: &Inventory,
    knows_hidden: bool,
    knows_packed: bool,
    knows_all: bool,
) -> InventoryView {
    let equipped_items = inventory
        .equipment
        .iter()
        .filter(|character_item| character_item.is_equipped())
        .filter_map(|character_item| {
            if !character_item.is_hidden || knows_hidden || knows_all {
                Some(super::character_item::look_at(
                    character_item,
                    knows_hidden,
                    knows_all,
                ))
            } else {
                None
            }
        });

    let packed_items = inventory
        .equipment
        .iter()
        .filter(|character_item| character_item.is_packed())
        .filter_map(|character_item| {
            if knows_packed || knows_all {
                Some(super::character_item::look_at(
                    character_item,
                    true,
                    knows_all,
                ))
            } else {
                None
            }
        });

    InventoryView {
        equipment: equipped_items.chain(packed_items).collect(),
    }
}
