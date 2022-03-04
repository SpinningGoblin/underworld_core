use enum_iterator::IntoEnumIterator;
use rand::{prelude::ThreadRng, Rng};
use uuid::Uuid;

use crate::components::{
    character::Character,
    damage::{Attack, Defense},
    identifier::Identifier,
    inventory::Inventory,
    items::{
        character_item::CharacterItem, descriptor::Descriptor, item::Item, item_type::ItemType,
        location_descriptor::LocationDescriptor, location_tag::LocationTag,
    },
    material::Material,
    player::PlayerCharacter,
    size::Size,
    species::Species,
    tag::Tag,
};

use super::{generator::Generator, stats::build_specific_health};

pub struct PlayerCharacterPrototype {
    pub username: String,
    pub character_name: Option<String>,
    pub species: Option<Species>,
    pub size: Option<Size>,
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
                let options: Vec<Species> = Species::into_enum_iter().collect();
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
                inventory: Some(Inventory {
                    equipment: starter_wearables
                        .into_iter()
                        .chain(vec![starter_weapon].into_iter())
                        .collect(),
                }),
            },
            identifier: Identifier {
                id: Uuid::new_v4(),
                name: self.character_name.clone(),
            },
            username: self.username.clone(),
        }
    }
}

fn starter_wearables() -> Vec<CharacterItem> {
    let trousers = Item {
        identifier: Identifier::just_id(),
        item_type: ItemType::Trousers,
        tags: vec![Tag::Clothing, Tag::Cloth],
        descriptors: vec![Descriptor::Dirty, Descriptor::Stained],
        material: Some(Material::Linen),
        attack: None,
        defense: Some(Defense {
            num_rolls: 1,
            modifier: -3,
        }),
    };

    let shirt = Item {
        identifier: Identifier::just_id(),
        item_type: ItemType::Shirt,
        tags: vec![Tag::Clothing, Tag::Cloth],
        descriptors: vec![Descriptor::Colourful],
        material: Some(Material::Linen),
        attack: None,
        defense: Some(Defense {
            num_rolls: 1,
            modifier: -3,
        }),
    };

    let boots = Item {
        identifier: Identifier::just_id(),
        item_type: ItemType::Boots,
        tags: vec![Tag::Clothing, Tag::Leather],
        descriptors: vec![Descriptor::Dirty],
        material: Some(Material::Leather),
        attack: None,
        defense: Some(Defense {
            num_rolls: 1,
            modifier: -3,
        }),
    };

    vec![
        CharacterItem {
            item: trousers,
            is_hidden: false,
            location_descriptor: LocationDescriptor::None,
            equipped_location_tags: vec![LocationTag::Equipped, LocationTag::Leg],
            is_multiple: false,
        },
        CharacterItem {
            item: shirt,
            is_hidden: false,
            location_descriptor: LocationDescriptor::None,
            equipped_location_tags: vec![LocationTag::Equipped, LocationTag::Body],
            is_multiple: false,
        },
        CharacterItem {
            item: boots,
            is_hidden: false,
            location_descriptor: LocationDescriptor::None,
            equipped_location_tags: vec![LocationTag::Equipped, LocationTag::Feet],
            is_multiple: false,
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
        identifier: Identifier::just_id(),
        item_type,
        tags,
        material,
        descriptors,
        attack: Some(Attack {
            num_rolls: 1,
            modifier: -1,
        }),
        defense: None,
    };

    CharacterItem {
        item,
        is_hidden: false,
        location_descriptor: LocationDescriptor::None,
        equipped_location_tags: vec![LocationTag::Equipped],
        is_multiple: false,
    }
}
