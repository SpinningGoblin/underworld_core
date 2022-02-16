#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

pub trait Tagged {
    fn tags(&self) -> Vec<Tag>;
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Tag {
    Accessory,
    Armour,
    Blade,
    Blunt,
    Bone,
    Cloth,
    Clothing,
    Combat,
    Container,
    Damage,
    Defense,
    Equipped,
    Fixture,
    Instrument,
    Leather,
    Metal,
    Rope,
    Shield,
    Stone,
    Whip,
    Wood,
}

impl Tag {
    pub fn is_weapon(&self) -> bool {
        vec![Tag::Blade, Tag::Blunt, Tag::Whip].contains(self)
    }

    pub fn is_wearable(&self) -> bool {
        vec![Tag::Accessory, Tag::Armour, Tag::Clothing].contains(self)
    }
}
