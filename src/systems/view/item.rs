use crate::components::items::{
    ConsumableView, ThrowableView, {Item, ItemView},
};

pub fn view(item: &Item, sees_full_item: bool, knows_all: bool) -> ItemView {
    let (descriptors, descriptors_known) = if sees_full_item || knows_all {
        (item.descriptors.clone(), true)
    } else {
        (Vec::new(), false)
    };

    let (material, material_known) = if sees_full_item || knows_all {
        (item.material, true)
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

    let (consumable, knows_consumable) = if knows_all {
        if let Some(c) = &item.consumable {
            (
                Some(ConsumableView {
                    uses: c.uses,
                    knows_uses: true,
                    effect: c.effect.clone(),
                    knows_effect: true,
                }),
                true,
            )
        } else {
            (None, true)
        }
    } else {
        (None, false)
    };

    let throwable = item.throwable.as_ref().map(|throwable| ThrowableView {
        uses: throwable.uses,
        effect: throwable.effect.clone(),
    });

    ItemView {
        id: item.id.to_string(),
        name: item.name.clone(),
        item_type: item.item_type,
        tags: item.tags.clone(),
        is_equippable: item.is_equippable(),
        descriptors,
        descriptors_known,
        material,
        material_known,
        attack,
        attack_known,
        defense,
        defense_known,
        consumable,
        knows_consumable,
        throwable,
    }
}
