#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

pub trait Tagged {
    fn tags(&self) -> Vec<Tag>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum Tag {
    Accessory,
    Armour,
    Blade,
    Blunt,
    Bone,
    Ceramic,
    Cloth,
    Clothing,
    Combat,
    Consumable,
    Container,
    Damage,
    Defense,
    Equippable,
    Fixture,
    Instrument,
    Leather,
    Metal,
    Paper,
    Rope,
    Shield,
    Stone,
    Teachable,
    Throwable,
    Thrust,
    Weapon,
    Wearable,
    Whip,
    Wood,
}

impl Tag {
    pub fn is_consumable(&self) -> bool {
        matches!(*self, Tag::Consumable | Tag::Teachable)
    }

    pub fn is_equippable(&self) -> bool {
        self.is_weapon() || self.is_wearable() || matches!(*self, Tag::Equippable)
    }

    pub fn is_weapon(&self) -> bool {
        matches!(
            *self,
            Tag::Blade | Tag::Blunt | Tag::Whip | Tag::Weapon | Tag::Thrust
        )
    }

    pub fn is_wearable(&self) -> bool {
        matches!(
            *self,
            Tag::Accessory | Tag::Armour | Tag::Clothing | Tag::Wearable
        )
    }
}
