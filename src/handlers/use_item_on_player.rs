use std::error::Error;

use chrono::Utc;

use crate::{
    actions::UseItemOnPlayer,
    components::{
        identifier::Identifier, items::consumable_effect::ConsumableEffectName,
        player::PlayerCharacter, spells::spell::Spell,
    },
    errors::{ItemNotDirectlyUsableError, ItemNotFoundError},
    events::{Event, PlayerItemRemoved, PlayerItemUsed, PlayerSpellLearned},
    utils::ids::parse_id,
};

pub fn handle(
    use_item_on_player: &UseItemOnPlayer,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Box<dyn Error>> {
    let item_id = parse_id(&use_item_on_player.item_id)?;
    let character_item = match player.character.find_item(&item_id) {
        Some(it) => it,
        None => return Err(Box::new(ItemNotFoundError(item_id.to_string()))),
    };

    if !character_item.is_consumable() {
        return Err(Box::new(ItemNotDirectlyUsableError(item_id.to_string())));
    }

    let consumable = match character_item.item.consumable {
        Some(it) => it,
        None => return Ok(Vec::new()),
    };

    let mut events = match consumable.effect.name {
        ConsumableEffectName::LearnSpell => {
            if let Some(learn_spell_effect) = consumable.effect.learn_spell_effect {
                let spell = Spell {
                    name: learn_spell_effect.spell_name.clone(),
                    attack: learn_spell_effect.spell_attack.clone(),
                    defense: learn_spell_effect.spell_defense.clone(),
                    uses: learn_spell_effect.spell_uses,
                };

                vec![Event::PlayerSpellLearned(PlayerSpellLearned {
                    spell_identifier: Identifier::just_id(),
                    learned_at: Utc::now(),
                    spell,
                })]
            } else {
                Vec::new()
            }
        }
    };

    events.push(Event::PlayerItemUsed(PlayerItemUsed { item_id }));

    if consumable.uses - 1 == 0 {
        events.push(Event::PlayerItemRemoved(PlayerItemRemoved { item_id }));
    }

    Ok(events)
}
