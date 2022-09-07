use rand::{prelude::ThreadRng, Rng};
use std::ops::RangeInclusive;
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{
    components::{
        items::{
            CharacterItem, Consumable, HealingEffect, Item, ItemType, LocationTag, OilSplashEffect,
            Throwable, ThrowableEffect, ThrowableEffectName,
            {ConsumableEffect, ConsumableEffectName, LearnSpellEffect},
        },
        spells::SpellName,
        Inventory, Tagged, {Attack, Defense},
    },
    utils::rolls::roll_percent_succeeds,
};

use super::{
    generator::Generator,
    items::item_generator_for_level,
    utils::item_types::{type_is_for_weapon, type_is_for_wearable},
};

#[derive(Default, Clone)]
pub struct InventoryGeneratorBuilder {
    possible_item_types: Option<Vec<ItemType>>,
    num_equipped_weapons: Option<RangeInclusive<u16>>,
    num_equipped_wearables: Option<RangeInclusive<u16>>,
    danger_level: Option<u32>,
    generate_consumable_chance: Option<i32>,
    generate_throwable_chance: Option<i32>,
}

const GENERATE_CONSUMABLE_CHANCE: i32 = 25;
const GENERATE_POT_CHANCE: i32 = 20;
const WEAPON_IN_HAND_CHANCE: i32 = 95;

impl InventoryGeneratorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn possible_item_types(&mut self, item_types: Vec<ItemType>) -> &mut Self {
        self.possible_item_types = Some(item_types);

        self
    }

    /// Set a range for the number of equipped weapons. Max is 2 weapons equipped.
    pub fn num_equipped_weapons(&mut self, num_equipped_weapons: RangeInclusive<u16>) -> &mut Self {
        let max = if num_equipped_weapons.end() > &2 {
            2
        } else {
            *num_equipped_weapons.end()
        };
        let min = if num_equipped_weapons.start() > &2 {
            2
        } else if num_equipped_weapons.start() > &max {
            max
        } else {
            *num_equipped_weapons.start()
        };
        self.num_equipped_weapons = Some(min..=max);
        self
    }

    /// Set a range for the number of equipped wearables. Max is 8.
    pub fn num_equipped_wearables(
        &mut self,
        num_equipped_wearables: RangeInclusive<u16>,
    ) -> &mut Self {
        let max = if num_equipped_wearables.end() > &8 {
            8
        } else {
            *num_equipped_wearables.end()
        };
        let min = if num_equipped_wearables.start() > &8 {
            8
        } else if num_equipped_wearables.start() > &max {
            max
        } else {
            *num_equipped_wearables.start()
        };
        self.num_equipped_wearables = Some(min..=max);
        self
    }

    pub fn danger_level(&mut self, danger_level: u32) -> &mut Self {
        self.danger_level = Some(danger_level);

        self
    }

    pub fn generate_consumable_chance(&mut self, generate_chance: i32) -> &mut Self {
        self.generate_consumable_chance = Some(generate_chance);

        self
    }

    pub fn generate_throwable_chance(&mut self, generate_chance: i32) -> &mut Self {
        self.generate_throwable_chance = Some(generate_chance);

        self
    }

    pub fn build(&self) -> impl Generator<Inventory> {
        let item_types = match &self.possible_item_types {
            Some(it) => it.clone(),
            None => ItemType::iter().collect(),
        };

        let num_equipped_weapons = match &self.num_equipped_weapons {
            Some(range) => range.clone(),
            None => 1..=2,
        };

        let num_equipped_wearables = match &self.num_equipped_wearables {
            Some(range) => range.clone(),
            None => 3..=8,
        };

        InventoryPrototype {
            item_types,
            num_equipped_weapons,
            num_equipped_wearables,
            danger_level: self.danger_level.unwrap_or(1),
            generate_consumable_chance: self
                .generate_consumable_chance
                .unwrap_or(GENERATE_CONSUMABLE_CHANCE),
            generate_throwable_chance: self
                .generate_throwable_chance
                .unwrap_or(GENERATE_POT_CHANCE),
        }
    }
}

struct InventoryPrototype {
    pub item_types: Vec<ItemType>,
    pub num_equipped_weapons: RangeInclusive<u16>,
    pub num_equipped_wearables: RangeInclusive<u16>,
    pub danger_level: u32,
    pub generate_consumable_chance: i32,
    pub generate_throwable_chance: i32,
}

impl InventoryPrototype {
    fn equipped_weapons(&self, rng: &mut ThreadRng) -> Vec<CharacterItem> {
        let count = rng.gen_range(self.num_equipped_weapons.clone());

        if count == 0 {
            return Vec::new();
        }

        let mut equipped_weapons: Vec<CharacterItem> = Vec::new();
        let weapon_types: Vec<&ItemType> = self
            .item_types
            .iter()
            .filter(|item_type| type_is_for_weapon(item_type))
            .collect();
        let mut location_tags = LocationTag::not_at_ready_weapon_tags();
        for _ in 1..=count {
            if location_tags.is_empty() {
                break;
            }

            let tag = if roll_percent_succeeds(rng, WEAPON_IN_HAND_CHANCE) {
                LocationTag::Hand
            } else {
                let tag_index = rng.gen_range(0..location_tags.len());
                location_tags.remove(tag_index)
            };

            let possible_weapon_types: Vec<&ItemType> = weapon_types
                .iter()
                .filter(|item_type| item_type_is_for_tags(item_type, &tag))
                .cloned()
                .collect();

            if possible_weapon_types.is_empty() {
                continue;
            }

            let index = rng.gen_range(0..possible_weapon_types.len());
            let weapon_type = match &possible_weapon_types.get(index) {
                Some(it) => *it,
                None => continue,
            };
            let generator = item_generator_for_level(weapon_type, true, self.danger_level);
            let weapon = generator.generate();

            equipped_weapons.push(CharacterItem {
                item: weapon,
                at_the_ready: tag.eq(&LocationTag::Hand),
                equipped_location: tag,
            })
        }

        equipped_weapons
    }

    fn equipped_wearables(&self, rng: &mut ThreadRng) -> Vec<CharacterItem> {
        let count = rng.gen_range(self.num_equipped_wearables.clone());

        if count == 0 {
            return Vec::new();
        }

        let mut equipped_wearables: Vec<CharacterItem> = Vec::new();
        let mut used_types: Vec<ItemType> = Vec::new();
        let mut wearable_tags = LocationTag::wearable_tags();

        for _ in 1..=count {
            if wearable_tags.is_empty() {
                break;
            }

            let tag_index = rng.gen_range(0..wearable_tags.len());
            let tag = wearable_tags.remove(tag_index);

            let possible_types: Vec<ItemType> = self
                .item_types
                .iter()
                .filter(|item_type| {
                    type_is_for_wearable(item_type) && item_type_is_for_tags(item_type, &tag)
                })
                .cloned()
                .collect();

            if possible_types.is_empty() {
                break;
            }

            let index = rng.gen_range(0..possible_types.len());
            let wearable_type = match &possible_types.get(index) {
                Some(it) => *it,
                _ => continue,
            };

            used_types.push(*wearable_type);
            let generator = item_generator_for_level(wearable_type, true, self.danger_level);
            let wearable = generator.generate();

            equipped_wearables.push(CharacterItem {
                item: wearable,
                at_the_ready: true,
                equipped_location: tag,
            })
        }

        equipped_wearables
    }

    fn spell_uses(&self, rng: &mut ThreadRng, spell_name: &SpellName) -> i32 {
        match *spell_name {
            SpellName::AcidSplash => rng.gen_range(1..=3),
            SpellName::ElectricBlast
            | SpellName::RagingFireball
            | SpellName::PoisonCloud
            | SpellName::PoisonDart
            | SpellName::TinyShield => rng.gen_range(1..=6),
            SpellName::Heal => rng.gen_range(1..=5),
            SpellName::GreatHeal | SpellName::Phoenix => 1,
            SpellName::QuickHeal => rng.gen_range(3..=10),
            SpellName::Retribution => rng.gen_range(2..=3),
        }
    }

    fn pots(&self, rng: &mut ThreadRng) -> Vec<CharacterItem> {
        let possible_materials = super::utils::materials::possible_materials(&ItemType::Pot);
        let material = if possible_materials.is_empty() {
            None
        } else {
            let material_index = rng.gen_range(0..possible_materials.len());
            possible_materials.get(material_index).cloned()
        };

        let covers_all_enemies = roll_percent_succeeds(rng, 90);

        let possible_descriptors = super::utils::item_descriptors::possible_descriptors(
            &ItemType::Pot,
            &material,
            self.danger_level,
        );
        let descriptors = if possible_descriptors.is_empty() {
            Vec::new()
        } else {
            let descriptor_index = rng.gen_range(0..possible_descriptors.len());
            let descriptor = possible_descriptors.get(descriptor_index).cloned().unwrap();
            vec![descriptor]
        };

        vec![CharacterItem {
            item: Item {
                id: Uuid::new_v4(),
                name: None,
                item_type: ItemType::Pot,
                tags: ItemType::Pot.tags(),
                descriptors,
                material,
                attack: None,
                defense: None,
                consumable: None,
                throwable: Some(Throwable {
                    uses: 1,
                    effect: ThrowableEffect {
                        name: ThrowableEffectName::OilSplash,
                        oil_splash_effect: Some(OilSplashEffect { covers_all_enemies }),
                    },
                }),
            },
            equipped_location: LocationTag::Packed,
            at_the_ready: false,
        }]
    }

    fn healing_grog_consumable(&self, rng: &mut ThreadRng) -> Consumable {
        let num_rolls = if (1..=10).contains(&self.danger_level) {
            1
        } else if (11..=25).contains(&self.danger_level) {
            2
        } else if (26..=40).contains(&self.danger_level) {
            3
        } else if (41..=60).contains(&self.danger_level) {
            4
        } else {
            5
        };
        let healing = Attack {
            num_rolls,
            modifier: 0,
            effects: Vec::new(),
        };

        let uses = rng.gen_range(1..=5);

        Consumable {
            effect: ConsumableEffect {
                name: ConsumableEffectName::HealingGrog,
                learn_spell_effect: None,
                healing_effect: Some(HealingEffect { healing }),
            },
            uses,
        }
    }

    fn spell_consumable(&self, rng: &mut ThreadRng) -> Consumable {
        let spell_names: Vec<SpellName> = SpellName::iter().collect();
        let index = rng.gen_range(0..spell_names.len());
        let spell_name = spell_names.get(index).unwrap();

        let spell_uses: i32 = self.spell_uses(rng, spell_name);

        let spell_attack = if matches!(
            spell_name,
            SpellName::RagingFireball | SpellName::ElectricBlast
        ) {
            let num_rolls = if (1..=10).contains(&self.danger_level) {
                2
            } else if (11..=25).contains(&self.danger_level) {
                5
            } else if (26..=40).contains(&self.danger_level) {
                9
            } else if (41..=60).contains(&self.danger_level) {
                15
            } else {
                20
            };
            Some(Attack {
                num_rolls,
                modifier: 0,
                effects: Vec::new(),
            })
        } else if spell_name == &SpellName::Retribution {
            let num_rolls = if (1..=10).contains(&self.danger_level) {
                3
            } else if (11..=25).contains(&self.danger_level) {
                4
            } else if (26..=40).contains(&self.danger_level) {
                6
            } else if (41..=60).contains(&self.danger_level) {
                8
            } else {
                10
            };

            Some(Attack {
                num_rolls,
                modifier: -1,
                effects: Vec::new(),
            })
        } else if spell_name == &SpellName::QuickHeal {
            let num_rolls = if (1..=10).contains(&self.danger_level) {
                1
            } else if (11..=25).contains(&self.danger_level) {
                2
            } else if (26..=40).contains(&self.danger_level) {
                3
            } else if (41..=60).contains(&self.danger_level) {
                4
            } else {
                5
            };
            Some(Attack {
                num_rolls,
                modifier: 0,
                effects: Vec::new(),
            })
        } else if spell_name == &SpellName::Heal {
            let num_rolls = if (1..=10).contains(&self.danger_level) {
                2
            } else if (11..=25).contains(&self.danger_level) {
                4
            } else if (26..=40).contains(&self.danger_level) {
                6
            } else if (41..=60).contains(&self.danger_level) {
                8
            } else {
                10
            };

            Some(Attack {
                num_rolls,
                modifier: 0,
                effects: Vec::new(),
            })
        } else {
            None
        };

        let spell_defense = if spell_name == &SpellName::TinyShield {
            let damage_resistance = if (1..=10).contains(&self.danger_level) {
                rng.gen_range(2..=10)
            } else if (11..=25).contains(&self.danger_level) {
                rng.gen_range(5..=20)
            } else if (26..=40).contains(&self.danger_level) {
                rng.gen_range(10..=30)
            } else if (41..=60).contains(&self.danger_level) {
                rng.gen_range(15..=40)
            } else {
                rng.gen_range(20..=50)
            };
            Some(Defense { damage_resistance })
        } else {
            None
        };

        Consumable {
            effect: ConsumableEffect {
                name: ConsumableEffectName::LearnSpell,
                learn_spell_effect: Some(LearnSpellEffect {
                    spell_name: *spell_name,
                    spell_attack,
                    spell_defense,
                    spell_uses,
                }),
                healing_effect: None,
            },
            uses: 1,
        }
    }

    fn consumables(&self, rng: &mut ThreadRng) -> Vec<CharacterItem> {
        let names: Vec<ConsumableEffectName> = ConsumableEffectName::iter().collect();
        let name_index = rng.gen_range(0..names.len());
        let consumable_name = names.get(name_index).cloned().unwrap();

        let consumable = match consumable_name {
            ConsumableEffectName::LearnSpell => self.spell_consumable(rng),
            ConsumableEffectName::HealingGrog => self.healing_grog_consumable(rng),
        };

        let item_type = match consumable_name {
            ConsumableEffectName::LearnSpell => ItemType::Scroll,
            ConsumableEffectName::HealingGrog => ItemType::Flask,
        };

        let possible_materials = super::utils::materials::possible_materials(&item_type);
        let material = if possible_materials.is_empty() {
            None
        } else {
            let material_index = rng.gen_range(0..possible_materials.len());
            possible_materials.get(material_index).cloned()
        };

        let possible_descriptors = super::utils::item_descriptors::possible_descriptors(
            &item_type,
            &material,
            self.danger_level,
        );
        let descriptors = if possible_descriptors.is_empty() {
            Vec::new()
        } else {
            let descriptor_index = rng.gen_range(0..possible_descriptors.len());
            let descriptor = possible_descriptors.get(descriptor_index).cloned().unwrap();
            vec![descriptor]
        };

        vec![CharacterItem {
            item: Item {
                id: Uuid::new_v4(),
                name: None,
                tags: item_type.tags(),
                item_type,
                descriptors,
                material,
                attack: None,
                defense: None,
                consumable: Some(consumable),
                throwable: None,
            },
            equipped_location: LocationTag::Packed,
            at_the_ready: false,
        }]
    }
}

impl Generator<Inventory> for InventoryPrototype {
    fn generate(&self) -> Inventory {
        let mut rng = rand::thread_rng();

        let equipped_weapons = self.equipped_weapons(&mut rng);
        let equipped_wearables = self.equipped_wearables(&mut rng);

        let consumables = if roll_percent_succeeds(&mut rng, self.generate_consumable_chance) {
            self.consumables(&mut rng)
        } else {
            Vec::new()
        };

        let pots = if roll_percent_succeeds(&mut rng, self.generate_throwable_chance) {
            self.pots(&mut rng)
        } else {
            Vec::new()
        };

        Inventory {
            equipment: equipped_weapons
                .into_iter()
                .chain(equipped_wearables.into_iter())
                .chain(consumables.into_iter())
                .chain(pots.into_iter())
                .collect(),
        }
    }
}

fn item_type_is_for_tags(item_type: &ItemType, tag: &LocationTag) -> bool {
    match *item_type {
        ItemType::Breastplate | ItemType::Shirt | ItemType::Vest => tag.eq(&LocationTag::Body),
        ItemType::Boots | ItemType::PlateBoots => tag.eq(&LocationTag::Feet),
        ItemType::Buckler => tag.eq(&LocationTag::Hand),
        ItemType::Cloak => tag.eq(&LocationTag::Shoulder),
        ItemType::Club
        | ItemType::Hammer
        | ItemType::Mace
        | ItemType::Morningstar
        | ItemType::Whip => tag.eq(&LocationTag::Hand) || tag.eq(&LocationTag::Hip),
        ItemType::Dagger | ItemType::ShortSword | ItemType::Dirk => {
            tag.eq(&LocationTag::Hand)
                || vec![LocationTag::Hip, LocationTag::HipSheath].contains(tag)
        }
        ItemType::Crown
        | ItemType::PlateHelmet
        | ItemType::Helm
        | ItemType::BowlerHat
        | ItemType::Fedora
        | ItemType::TopHat => tag.eq(&LocationTag::Head),
        ItemType::Gloves | ItemType::PlateGauntlets => tag.eq(&LocationTag::Hand),
        ItemType::GreatSword
        | ItemType::Halberd
        | ItemType::Pike
        | ItemType::Shield
        | ItemType::Spear => tag.eq(&LocationTag::Hand) || tag.eq(&LocationTag::Back),
        ItemType::LoinCloth => tag.eq(&LocationTag::Waist),
        ItemType::LongSword => {
            tag.eq(&LocationTag::Hand)
                || vec![LocationTag::Hip, LocationTag::HipSheath].contains(tag)
                || tag.eq(&LocationTag::Back)
        }
        ItemType::Mask => tag.eq(&LocationTag::Face),
        ItemType::Shackles => tag.eq(&LocationTag::Wrist) | tag.eq(&LocationTag::Ankle),
        ItemType::Trousers => tag.eq(&LocationTag::Leg),
        ItemType::Scroll => tag.eq(&LocationTag::Packed) | tag.eq(&LocationTag::Pockets),
        ItemType::Pot => tag.eq(&LocationTag::Packed),
        ItemType::Flask => tag.eq(&LocationTag::Packed),
    }
}
