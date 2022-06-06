use std::error::Error;

use crate::{
    actions::CastSpellOnPlayer,
    components::{
        damage::{Attack, Defense},
        player::PlayerCharacter,
        spells::spell_name::SpellName,
    },
    errors::SpellNotFoundError,
    events::{
        Event, PlayerGainsResurrectionAura, PlayerGainsRetributionAura, PlayerGainsShieldAura,
        PlayerHealed, PlayerHit, PlayerSpellForgotten, PlayerSpellUsed,
    },
    utils::ids::parse_id,
};

pub fn handle(
    cast_spell_on_player: &CastSpellOnPlayer,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Box<dyn Error>> {
    let spell_id = parse_id(&cast_spell_on_player.spell_id)?;
    let learned_spell = match player.character.find_spell(&spell_id) {
        Some(it) => it,
        None => return Err(Box::new(SpellNotFoundError(spell_id.to_string()))),
    };

    let mut events: Vec<Event> = Vec::new();
    match learned_spell.spell.name {
        SpellName::ElectricBlast | SpellName::RagingFireball => {
            let damage = learned_spell.spell.damage();
            events.push(Event::PlayerHit(PlayerHit {
                player_id: player.id,
                attacker_id: player.id,
                damage,
            }));
        }
        SpellName::Heal | SpellName::QuickHeal => {
            let damage_healed = learned_spell.spell.damage();
            events.push(Event::PlayerHealed(PlayerHealed { damage_healed }));
        }
        SpellName::Phoenix => events.push(Event::PlayerGainsResurrectionAura(
            PlayerGainsResurrectionAura,
        )),
        SpellName::Retribution => {
            let attack = learned_spell.spell.attack.clone().unwrap_or(Attack {
                num_rolls: 2,
                modifier: 0,
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
    }

    events.push(Event::PlayerSpellUsed(PlayerSpellUsed { spell_id }));

    if learned_spell.spell.uses - 1 == 0 {
        events.push(Event::PlayerSpellForgotten(PlayerSpellForgotten {
            spell_id,
        }));
    }

    Ok(events)
}
