use chrono::Utc;
use uuid::Uuid;

use crate::{
    actions::UseItemOnPlayer,
    components::{items::ConsumableEffectName, spells::Spell, PlayerCharacter},
    errors::Error,
    events::{Event, PlayerHealed, PlayerItemRemoved, PlayerItemUsed, PlayerSpellLearned},
    utils::ids::parse_id,
};

pub fn handle(
    use_item_on_player: &UseItemOnPlayer,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Error> {
    let item_id = parse_id(&use_item_on_player.item_id)?;
    let character_item = match player.character.find_item(&item_id) {
        Some(it) => it,
        None => return Err(Error::ItemNotFoundError(item_id.to_string())),
    };

    if !character_item.is_consumable() {
        return Err(Error::ItemNotDirectlyUsableError(item_id.to_string()));
    }

    let consumable = match character_item.item.consumable {
        Some(it) => it,
        None => return Ok(Vec::new()),
    };

    let mut events = match consumable.effect.name {
        ConsumableEffectName::LearnSpell => {
            if let Some(learn_spell_effect) = consumable.effect.learn_spell_effect {
                let spell = Spell {
                    name: learn_spell_effect.spell_name,
                    attack: learn_spell_effect.spell_attack.clone(),
                    defense: learn_spell_effect.spell_defense.clone(),
                    uses: learn_spell_effect.spell_uses,
                };

                vec![Event::PlayerSpellLearned(PlayerSpellLearned {
                    learned_at: Utc::now(),
                    spell,
                    spell_id: Uuid::new_v4(),
                })]
            } else {
                Vec::new()
            }
        }
        ConsumableEffectName::HealingGrog => {
            if let Some(heal_effect) = consumable.effect.healing_effect {
                let mut rng = rand::thread_rng();
                let healing = heal_effect.healing.attack_roll(&mut rng);
                let damage_healed = healing
                    .min(player.character.stats.health.max - player.character.stats.health.current);
                vec![Event::PlayerHealed(PlayerHealed { damage_healed })]
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
