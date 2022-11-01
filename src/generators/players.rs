use chrono::Utc;
use rand::{prelude::ThreadRng, Rng};
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::components::{
    items::{CharacterItem, Descriptor, Item, ItemType, LocationTag},
    spells::{LearnedSpell, Spell, SpellMemory, SpellName},
    Character, Effects, Inventory, Material, PlayerCharacter, Size, Species, Stats, Tag,
    {Attack, Defense},
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

        let species = match &self.species {
            Some(it) => *it,
            None => {
                let options: Vec<Species> = Species::iter().collect();
                let index = rng.gen_range(0..options.len());
                *options.get(index).unwrap()
            }
        };

        let stats_generator = build_specific_health(25, &species, false);
        let mut stats: Stats = stats_generator.generate();

        if let Some(size) = &self.size {
            stats.height = *size;
        }

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
            gold: 0,
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
        throwable: None,
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
        throwable: None,
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
        throwable: None,
    };

    vec![
        CharacterItem {
            item: trousers,
            at_the_ready: true,
            equipped_location: LocationTag::Leg,
        },
        CharacterItem {
            item: shirt,
            at_the_ready: true,
            equipped_location: LocationTag::Body,
        },
        CharacterItem {
            item: boots,
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
    let item_type = *weapon_types.get(index).unwrap_or(&ItemType::Dagger);

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
            effects: Vec::new(),
        }),
        defense: None,
        consumable: None,
        throwable: None,
    };

    CharacterItem {
        item,
        equipped_location: LocationTag::Hand,
        at_the_ready: true,
    }
}
