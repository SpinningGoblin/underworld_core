use rand::Rng;

use crate::components::weapon::EquippedWeapon;

use super::{generator::Generator, weapons::WeaponPrototype};

pub struct EquippedWeaponPrototype {
    pub weapon_prototype: WeaponPrototype,
    pub hidden_chance: usize,
    pub multiple: bool,
    pub equipped_locations: Vec<String>,
    pub equipped_location_chance: usize,
}

impl EquippedWeaponPrototype {
    pub fn dagger(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            weapon_prototype: WeaponPrototype::dagger(),
            multiple: false,
            equipped_locations: vec![
                "hanging in a moldy sheath".to_string(),
                "strapped around its thigh".to_string(),
                "clenched in its fist".to_string(),
            ],
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
            weapon_prototype: WeaponPrototype::long_sword(),
            multiple: false,
            equipped_locations: vec![
                "hanging in a moldy sheath".to_string(),
                "clenched in its fist".to_string(),
                "strapped to its back".to_string(),
            ],
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
            weapon_prototype: WeaponPrototype::short_sword(),
            multiple: false,
            equipped_locations: vec![
                "hanging in a moldy sheath".to_string(),
                "clenched in its fist".to_string(),
                "in its fist".to_string(),
            ],
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
            weapon_prototype: WeaponPrototype::club(),
            multiple: false,
            equipped_locations: vec![
                "hanging from its hip".to_string(),
                "clenched in its fist".to_string(),
            ],
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
            weapon_prototype: WeaponPrototype::hammer(),
            multiple: false,
            equipped_locations: vec![
                "hanging from its hip".to_string(),
                "clenched in its fist".to_string(),
            ],
        }
    }

    pub fn visible_hammer(equipped_location_chance: usize) -> Self {
        Self::hammer(0, equipped_location_chance)
    }

    pub fn hidden_hammer(equipped_location_chance: usize) -> Self {
        Self::hammer(100, equipped_location_chance)
    }
}

impl Generator<EquippedWeapon> for EquippedWeaponPrototype {
    fn generate(&self) -> EquippedWeapon {
        let weapon = self.weapon_prototype.generate();

        let mut rng = rand::thread_rng();
        let hidden_roll: usize = rng.gen_range(0..=100);

        let equipped_location_roll: usize = rng.gen_range(0..=100);
        let equipped_location = if equipped_location_roll <= self.equipped_location_chance {
            let index = rng.gen_range(0..self.equipped_locations.len());
            match self.equipped_locations.get(index) {
                Some(equipped_location) => equipped_location.clone(),
                _ => "".to_string(),
            }
        } else {
            "".to_string()
        };

        EquippedWeapon {
            weapon,
            equipped_location,
            hidden: hidden_roll <= self.hidden_chance,
            multiple: self.multiple,
        }
    }
}

#[cfg(test)]
mod equipped_weapon_generator_tests {
    use crate::generators::generator::Generator;

    use super::EquippedWeaponPrototype;

    #[test]
    fn dagger_generates() {
        let prototype = EquippedWeaponPrototype::dagger(25, 50);
        let equipped_dagger = prototype.generate();

        let weapon_description = format!("{}", equipped_dagger.weapon);
        assert!(weapon_description.contains("dagger"));
    }

    #[test]
    fn dagger_with_guaranteed_equipped_location() {
        let prototype = EquippedWeaponPrototype::dagger(25, 100);
        let equipped_dagger = prototype.generate();

        assert!(!equipped_dagger.equipped_location.is_empty());
    }

    #[test]
    fn dagger_with_guaranteed_hidden() {
        let prototype = EquippedWeaponPrototype::dagger(100, 50);
        let equipped_dagger = prototype.generate();

        assert!(equipped_dagger.hidden);
    }
}
