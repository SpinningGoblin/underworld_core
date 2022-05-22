use crate::components::items::item::{Item, ItemView};

pub fn view(item: &Item, sees_full_item: bool, knows_all: bool) -> ItemView {
    let (descriptors, descriptors_known) = if sees_full_item || knows_all {
        (item.descriptors.clone(), true)
    } else {
        (Vec::new(), false)
    };

    let (material, material_known) = if sees_full_item || knows_all {
        (item.material.clone(), true)
    } else {
        (None, false)
    };

    let (attack, attack_known) = if knows_all {
        (item.attack.clone(), true)
    } else {
        (None, false)
    };

    let (defense, defense_known) = if knows_all {
        (item.defense.clone(), true)
    } else {
        (None, false)
    };

    ItemView {
        identifier: super::identifier::view(&item.identifier, true),
        item_type: item.item_type.clone(),
        tags: item.tags.clone(),
        descriptors,
        descriptors_known,
        material,
        material_known,
        attack,
        attack_known,
        defense,
        defense_known,
    }
}
