use rand::Rng;

use crate::{
    actions::CastSpellOnPlayer,
    components::{
        spells::SpellName,
        PlayerCharacter, {Attack, Defense},
    },
    errors::Error,
    events::{
        Event, PlayerGainsRetributionAura, PlayerGainsShieldAura, PlayerHealed, PlayerHit,
        PlayerPoisoned, PlayerSpellForgotten, PlayerSpellUsed,
    },
    utils::{ids::parse_id, rolls::roll_percent_succeeds},
};

const ACID_DESTROYS_ITEM_CHANCE: i32 = 75;

pub fn handle(
    cast_spell_on_player: &CastSpellOnPlayer,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Error> {
    let spell_id = parse_id(&cast_spell_on_player.spell_id)?;
    let learned_spell = match player.character.find_spell(&spell_id) {
        Some(it) => it,
        None => return Err(Error::SpellNotFoundError(spell_id.to_string())),
    };

    let mut events: Vec<Event> = Vec::new();

    events.push(Event::PlayerSpellUsed(PlayerSpellUsed { spell_id }));

    match learned_spell.spell.name {
        SpellName::ElectricBlast | SpellName::RagingFireball => {
            let damage = learned_spell.spell.damage();
            events.push(Event::PlayerHit(PlayerHit {
                attacker_id: player.id,
                damage,
            }));
        }
        SpellName::Heal | SpellName::QuickHeal => {
            let healing = learned_spell.spell.damage();
            let damage_healed = healing
                .min(player.character.stats.health.max - player.character.stats.health.current);
            events.push(Event::PlayerHealed(PlayerHealed { damage_healed }));
        }
        SpellName::Phoenix => events.push(Event::PlayerGainsResurrectionAura),
        SpellName::Retribution => {
            let attack = learned_spell.spell.attack.clone().unwrap_or(Attack {
                num_rolls: 2,
                modifier: 0,
                effects: Vec::new(),
            });
            events.push(Event::PlayerGainsRetributionAura(
                PlayerGainsRetributionAura { attack },
            ));
        }
        SpellName::TinyShield => {
            let defense = learned_spell.spell.defense.clone().unwrap_or(Defense {
                damage_resistance: 6,
            });
            events.push(Event::PlayerGainsShieldAura(PlayerGainsShieldAura {
                defense,
            }));
        }
        SpellName::GreatHeal => {
            events.push(Event::PlayerHealthFullyRestored);
        }
        SpellName::PoisonCloud | SpellName::PoisonDart => {
            if player.character.current_effects.poison.is_none() {
                events.push(Event::PlayerPoisoned(PlayerPoisoned {
                    damage: 1,
                    duration: 1,
                }));
            } else {
                events.push(Event::PlayerPoisonLevelChanged(1));
                events.push(Event::PlayerPoisonDurationChanged(1));
            }
        }
        SpellName::AcidSplash => {
            let mut rng = rand::thread_rng();
            if roll_percent_succeeds(&mut rng, ACID_DESTROYS_ITEM_CHANCE) {
                let equipped_items = player.character.inventory.readied_weapons();
                let index = rng.gen_range(0..equipped_items.len());
                if let Some(character_item) = equipped_items.get(index) {
                    events.push(Event::PlayerHitWithAcid);
                    events.push(Event::PlayerItemDestroyed(character_item.item.id));
                }
            }
        }
    }

    if learned_spell.spell.uses - 1 == 0 {
        events.push(Event::PlayerSpellForgotten(PlayerSpellForgotten {
            spell_id,
        }));
    }

    Ok(events)
}
