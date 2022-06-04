use chrono::Utc;
use rand::{prelude::ThreadRng, Rng};
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::components::{
    character::Character,
    damage::{Attack, Defense},
    effects::Effects,
    inventory::Inventory,
    items::{
        character_item::CharacterItem, descriptor::Descriptor, item::Item, item_type::ItemType,
        location_tag::LocationTag,
    },
    material::Material,
    player::PlayerCharacter,
    size::Size,
    species::Species,
    spells::{
        learned_spell::LearnedSpell, spell::Spell, spell_memory::SpellMemory, spell_name::SpellName,
    },
    tag::Tag,
};

use super::{generator::Generator, stats::build_specific_health};

struct PlayerCharacterPrototype {
    pub character_name: Option<String>,
    pub species: Option<Species>,
    pub size: Option<Size>,
}

pub fn player_generator(
    character_name: Option<String>,
    species: Option<Species>,
    size: Option<Size>,
) -> impl Generator<PlayerCharacter> {
    PlayerCharacterPrototype {
        character_name,
        species,
        size,
    }
}

impl Generator<PlayerCharacter> for PlayerCharacterPrototype {
    fn generate(&self) -> PlayerCharacter {
        let mut rng = rand::thread_rng();
        let stats_generator = build_specific_health(25);
        let mut stats = stats_generator.generate();

        if let Some(size) = &self.size {
            stats.height = size.clone();
        }

        let species = match &self.species {
            Some(it) => it.clone(),
            None => {
                let options: Vec<Species> = Species::iter().collect();
                let index = rng.gen_range(0..options.len());
                options.get(index).unwrap().clone()
            }
        };

        let starter_weapon = starter_weapon(&mut rng);
        let starter_wearables = starter_wearables();

        PlayerCharacter {
            character: Character {
                stats,
                species,
                life_modifier: None,
                inventory: Inventory {
                    equipment: starter_wearables
                        .into_iter()
                        .chain(vec![starter_weapon].into_iter())
                        .collect(),
                },
                current_effects: Effects::default(),
                spell_memory: SpellMemory {
                    spells: vec![LearnedSpell {
                        id: Uuid::new_v4(),
                        spell: Spell {
                            name: SpellName::Phoenix,
                            attack: None,
                            defense: None,
                            uses: 1,
                        },
                        learned_at: Utc::now(),
                    }],
                },
            },
            id: Uuid::new_v4(),
            name: self.character_name.clone(),
        }
    }
}

fn starter_wearables() -> Vec<CharacterItem> {
    let trousers = Item {
        id: Uuid::new_v4(),
        name: None,
        item_type: ItemType::Trousers,
        tags: vec![Tag::Clothing, Tag::Cloth],
        descriptors: vec![Descriptor::Dirty, Descriptor::Stained],
        material: Some(Material::Linen),
        attack: None,
        defense: Some(Defense {
            damage_resistance: 1,
        }),
        consumable: None,
    };

    let shirt = Item {
        id: Uuid::new_v4(),
        name: None,
        item_type: ItemType::Shirt,
        tags: vec![Tag::Clothing, Tag::Cloth],
        descriptors: vec![Descriptor::Colourful],
        material: Some(Material::Linen),
        attack: None,
        defense: Some(Defense {
            damage_resistance: 1,
        }),
        consumable: None,
    };

    let boots = Item {
        id: Uuid::new_v4(),
        name: None,
        item_type: ItemType::Boots,
        tags: vec![Tag::Clothing, Tag::Leather],
        descriptors: vec![Descriptor::Dirty],
        material: Some(Material::Leather),
        attack: None,
        defense: Some(Defense {
            damage_resistance: 1,
        }),
        consumable: None,
    };

    vec![
        CharacterItem {
            item: trousers,
            is_hidden: false,
            is_multiple: false,
            at_the_ready: true,
            equipped_location: LocationTag::Leg,
        },
        CharacterItem {
            item: shirt,
            is_hidden: false,
            is_multiple: false,
            at_the_ready: true,
            equipped_location: LocationTag::Body,
        },
        CharacterItem {
            item: boots,
            is_hidden: false,
            is_multiple: false,
            at_the_ready: true,
            equipped_location: LocationTag::Feet,
        },
    ]
}

fn starter_weapon(rng: &mut ThreadRng) -> CharacterItem {
    let weapon_types: Vec<ItemType> = vec![
        ItemType::Dagger,
        ItemType::Dirk,
        ItemType::ShortSword,
        ItemType::Club,
    ];

    let index = rng.gen_range(0..weapon_types.len());
    let item_type = weapon_types.get(index).unwrap_or(&ItemType::Dagger).clone();

    let tags = if item_type == ItemType::Club {
        vec![Tag::Blunt]
    } else {
        vec![Tag::Blade]
    };

    let material = if item_type == ItemType::Club {
        Some(Material::Wooden)
    } else {
        Some(Material::Iron)
    };

    let descriptors = if item_type == ItemType::Club {
        vec![Descriptor::Beaten]
    } else {
        vec![Descriptor::Rusty]
    };

    let item = Item {
        id: Uuid::new_v4(),
        name: None,
        item_type,
        tags,
        material,
        descriptors,
        attack: Some(Attack {
            num_rolls: 1,
            modifier: -1,
        }),
        defense: None,
        consumable: None,
    };

    CharacterItem {
        item,
        is_hidden: false,
        equipped_location: LocationTag::Hand,
        is_multiple: false,
        at_the_ready: true,
    }
}
