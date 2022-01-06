use crate::describable::Describable;

#[derive(Clone, Debug)]
pub struct EquippedWeapon {
    pub weapon: Weapon,
    pub hidden: bool,
    pub equipped_location: String,
}

#[derive(Clone, Debug)]
pub struct Weapon {
    pub min_damage: i32,
    pub max_damage: i32,
    pub weapon_type: WeaponType,
    pub qualities: Vec<WeaponQuality>,
}

impl Describable for Weapon {
    fn describe(&self) -> String {
        let mut descriptions: Vec<String> = Vec::new();
        for quality in self.qualities.iter() {
            descriptions.push(quality.describe());
        }
        descriptions.push(self.weapon_type.describe());

        descriptions.join(" ")
    }
}

#[derive(Clone, Debug)]
pub enum WeaponQuality {
    Broken,
    Chipped,
    Dull,
    Rusty,
    Shiny,
}

impl Describable for WeaponQuality {
    fn describe(&self) -> String {
        match *self {
            Self::Broken => "broken".to_string(),
            Self::Chipped => "chipped".to_string(),
            Self::Dull => "dull".to_string(),
            Self::Rusty => "rusty".to_string(),
            Self::Shiny => "shiny".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum WeaponType {
    Club,
    Dagger,
    Hammer,
    Kukri,
    LongSword,
    Scimitar,
    ShortSword,
}

impl Describable for WeaponType {
    fn describe(&self) -> String {
        match *self {
            Self::Club => "club".to_string(),
            Self::Dagger => "dagger".to_string(),
            Self::Hammer => "hammer".to_string(),
            Self::Kukri => "kukri".to_string(),
            Self::LongSword => "long sword".to_string(),
            Self::Scimitar => "scimitar".to_string(),
            Self::ShortSword => "short sword".to_string(),
        }
    }
}

#[cfg(test)]
mod weapon_type_tests {
    use crate::{components::weapon::WeaponType, describable::Describable};

    #[test]
    fn describe() {
        assert_eq!("club", WeaponType::Club.describe());
        assert_eq!("dagger", WeaponType::Dagger.describe());
        assert_eq!("hammer", WeaponType::Hammer.describe());
        assert_eq!("kukri", WeaponType::Kukri.describe());
        assert_eq!("long sword", WeaponType::LongSword.describe());
        assert_eq!("scimitar", WeaponType::Scimitar.describe());
        assert_eq!("short sword", WeaponType::ShortSword.describe());
    }
}

#[cfg(test)]
mod weapon_quality_tests {
    use crate::{components::weapon::WeaponQuality, describable::Describable};

    #[test]
    fn describe() {
        assert_eq!("broken", WeaponQuality::Broken.describe());
        assert_eq!("chipped", WeaponQuality::Chipped.describe());
        assert_eq!("dull", WeaponQuality::Dull.describe());
        assert_eq!("rusty", WeaponQuality::Rusty.describe());
        assert_eq!("shiny", WeaponQuality::Shiny.describe());
    }
}

#[cfg(test)]
mod weapon_tests {
    use crate::describable::Describable;

    use super::Weapon;

    #[test]
    fn describe_without_qualities() {
        let weapon = Weapon {
            min_damage: 2,
            max_damage: 6,
            weapon_type: super::WeaponType::LongSword,
            qualities: Vec::new(),
        };

        assert_eq!("long sword", weapon.describe());
    }

    #[test]
    fn describe_with_qualities() {
        let weapon = Weapon {
            min_damage: 2,
            max_damage: 6,
            weapon_type: super::WeaponType::LongSword,
            qualities: vec![super::WeaponQuality::Dull, super::WeaponQuality::Chipped],
        };

        assert_eq!("dull chipped long sword", weapon.describe());
    }
}
