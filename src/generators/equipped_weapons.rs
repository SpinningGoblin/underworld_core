use crate::components::weapon::Weapon;

use super::{equipped_items::EquippedItemPrototype, weapons::WeaponPrototype};

impl EquippedItemPrototype<Weapon> {
    pub fn dagger(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WeaponPrototype::dagger()),
            multiple: false,
        }
    }

    pub fn visible_dagger(equipped_location_chance: usize) -> Self {
        Self::dagger(0, equipped_location_chance)
    }

    pub fn hidden_dagger(equipped_location_chance: usize) -> Self {
        Self::dagger(100, equipped_location_chance)
    }

    pub fn long_sword(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WeaponPrototype::long_sword()),
            multiple: false,
        }
    }

    pub fn visible_long_sword(equipped_location_chance: usize) -> Self {
        Self::long_sword(0, equipped_location_chance)
    }

    pub fn hidden_long_sword(equipped_location_chance: usize) -> Self {
        Self::long_sword(100, equipped_location_chance)
    }

    pub fn short_sword(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WeaponPrototype::short_sword()),
            multiple: false,
        }
    }

    pub fn visible_short_sword(equipped_location_chance: usize) -> Self {
        Self::short_sword(0, equipped_location_chance)
    }

    pub fn hidden_short_sword(equipped_location_chance: usize) -> Self {
        Self::short_sword(100, equipped_location_chance)
    }

    pub fn club(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WeaponPrototype::club()),
            multiple: false,
        }
    }

    pub fn visible_club(equipped_location_chance: usize) -> Self {
        Self::club(0, equipped_location_chance)
    }

    pub fn hidden_club(equipped_location_chance: usize) -> Self {
        Self::club(100, equipped_location_chance)
    }

    pub fn hammer(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WeaponPrototype::hammer()),
            multiple: false,
        }
    }

    pub fn visible_hammer(equipped_location_chance: usize) -> Self {
        Self::hammer(0, equipped_location_chance)
    }

    pub fn hidden_hammer(equipped_location_chance: usize) -> Self {
        Self::hammer(100, equipped_location_chance)
    }
}

#[cfg(test)]
mod equipped_weapon_generator_tests {
    use crate::generators::{equipped_items::EquippedItemPrototype, generator::Generator};

    #[test]
    fn dagger_generates() {
        let prototype = EquippedItemPrototype::dagger(25, 50);
        let equipped_dagger = prototype.generate();

        let weapon_description = format!("{}", equipped_dagger.item);
        assert!(weapon_description.contains("dagger"));
    }

    #[test]
    fn dagger_with_guaranteed_hidden() {
        let prototype = EquippedItemPrototype::dagger(100, 50);
        let equipped_dagger = prototype.generate();

        assert!(equipped_dagger.hidden);
    }
}
