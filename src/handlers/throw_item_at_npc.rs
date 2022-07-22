use uuid::Uuid;

use crate::{
    actions::ThrowItemAtNpc,
    components::{
        games::GameState,
        items::{Item, ItemType},
        rooms::Room,
        PlayerCharacter,
    },
    errors::Error,
    events::{Event, PlayerItemRemoved},
    utils::ids::parse_id,
};

pub fn handle(
    throw_item_at_npc: &ThrowItemAtNpc,
    state: &GameState,
    player: &PlayerCharacter,
) -> Result<Vec<Event>, Error> {
    let item_id = parse_id(&throw_item_at_npc.item_id)?;
    let character_item = match player.character.find_item(&item_id) {
        Some(it) => it,
        None => return Err(Error::ItemNotFoundError(item_id.to_string())),
    };

    if !character_item.item.is_throwable {
        return Err(Error::ItemNotThrowableError(item_id.to_string()));
    }

    let room = state.current_room();
    let npc_id = parse_id(&throw_item_at_npc.npc_id)?;
    match room.find_npc(&npc_id) {
        Some(it) => it,
        None => return Err(Error::NpcNotFoundError(npc_id.to_string())),
    };

    let mut events: Vec<Event> = Vec::new();

    match character_item.item.item_type {
        ItemType::Breastplate
        | ItemType::Boots
        | ItemType::BowlerHat
        | ItemType::Buckler
        | ItemType::Cloak
        | ItemType::Club
        | ItemType::Crown
        | ItemType::Dagger
        | ItemType::Dirk
        | ItemType::Fedora
        | ItemType::Gloves
        | ItemType::GreatSword
        | ItemType::Halberd
        | ItemType::Hammer
        | ItemType::Helm
        | ItemType::LoinCloth
        | ItemType::LongSword
        | ItemType::Mace
        | ItemType::Mask
        | ItemType::Morningstar
        | ItemType::Pike
        | ItemType::PlateBoots
        | ItemType::PlateGauntlets
        | ItemType::PlateHelmet
        | ItemType::Scroll
        | ItemType::Shield
        | ItemType::ShortSword
        | ItemType::Shirt
        | ItemType::Shackles
        | ItemType::Spear
        | ItemType::TopHat
        | ItemType::Trousers
        | ItemType::Vest
        | ItemType::Whip => {
            return Ok(Vec::new());
        }
        ItemType::Pot => {
            events.append(&mut handle_thrown_pot(room, &character_item.item, &npc_id));
        }
    }

    events.push(Event::PlayerItemRemoved(PlayerItemRemoved { item_id }));

    Ok(events)
}

fn handle_thrown_pot(room: &Room, item: &Item, target_npc_id: &Uuid) -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();

    if let Some(consumable) = &item.consumable {
        if let Some(oil_effect) = &consumable.effect.oil_splash_effect {
            if oil_effect.covers_all_enemies {
                for npc_position in room.npc_positions.iter() {
                    events.push(Event::NpcCoveredInOil(npc_position.npc.id));
                }
            } else {
                events.push(Event::NpcCoveredInOil(*target_npc_id));
            }
        }
    }

    events
}
