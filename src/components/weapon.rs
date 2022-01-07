use std::fmt::Display;

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

impl Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut descriptions: Vec<String> = Vec::new();
        for quality in self.qualities.iter() {
            descriptions.push(quality.to_string());
        }
        descriptions.push(self.weapon_type.to_string());

        write!(f, "{}", descriptions.join(" "))
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

impl Display for WeaponQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Broken => write!(f, "broken"),
            Self::Chipped => write!(f, "chipped"),
            Self::Dull => write!(f, "dull"),
            Self::Rusty => write!(f, "rusty"),
            Self::Shiny => write!(f, "shiny"),
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

impl Display for WeaponType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Club => write!(f, "club"),
            Self::Dagger => write!(f, "dagger"),
            Self::Hammer => write!(f, "hammer"),
            Self::Kukri => write!(f, "kukri"),
            Self::LongSword => write!(f, "long sword"),
            Self::Scimitar => write!(f, "scimitar"),
            Self::ShortSword => write!(f, "short sword"),
        }
    }
}

#[cfg(test)]
mod weapon_type_tests {
    use crate::components::weapon::WeaponType;

    #[test]
    fn to_string() {
        assert_eq!("club", WeaponType::Club.to_string());
        assert_eq!("dagger", WeaponType::Dagger.to_string());
        assert_eq!("hammer", WeaponType::Hammer.to_string());
        assert_eq!("kukri", WeaponType::Kukri.to_string());
        assert_eq!("long sword", WeaponType::LongSword.to_string());
        assert_eq!("scimitar", WeaponType::Scimitar.to_string());
        assert_eq!("short sword", WeaponType::ShortSword.to_string());
    }
}

#[cfg(test)]
mod weapon_quality_tests {
    use crate::components::weapon::WeaponQuality;

    #[test]
    fn to_string() {
        assert_eq!("broken", WeaponQuality::Broken.to_string());
        assert_eq!("chipped", WeaponQuality::Chipped.to_string());
        assert_eq!("dull", WeaponQuality::Dull.to_string());
        assert_eq!("rusty", WeaponQuality::Rusty.to_string());
        assert_eq!("shiny", WeaponQuality::Shiny.to_string());
    }
}

#[cfg(test)]
mod weapon_tests {
    use super::Weapon;

    #[test]
    fn to_string_without_qualities() {
        let weapon = Weapon {
            min_damage: 2,
            max_damage: 6,
            weapon_type: super::WeaponType::LongSword,
            qualities: Vec::new(),
        };

        assert_eq!("long sword", weapon.to_string());
    }

    #[test]
    fn to_string_with_qualities() {
        let weapon = Weapon {
            min_damage: 2,
            max_damage: 6,
            weapon_type: super::WeaponType::LongSword,
            qualities: vec![super::WeaponQuality::Dull, super::WeaponQuality::Chipped],
        };

        assert_eq!("dull chipped long sword", weapon.to_string());
    }
}
