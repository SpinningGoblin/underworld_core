#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use crate::components::items::item::{Item, ItemView};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct FixtureItem {
    pub item: Item,
    pub is_hidden: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "FixtureItem"))]
pub struct FixtureItemView {
    pub item: ItemView,
    pub is_hidden: bool,
    pub is_hidden_known: bool,
}

#[cfg(test)]
mod display_tests {
    use uuid::Uuid;

    use crate::{
        components::{
            fixtures::{fixture::FixtureViewArgs, fixture_type::FixtureType},
            material::Material,
        },
        systems::view::fixture::view,
    };

    use crate::components::fixtures::fixture::Fixture;

    #[test]
    fn display_with_material() {
        let fixture = Fixture {
            fixture_type: FixtureType::Chest,
            material: Some(Material::Steel),
            size: crate::components::size::Size::Average,
            descriptors: Vec::new(),
            id: Uuid::new_v4(),
            name: None,
            items: Vec::new(),
            has_hidden_compartment: false,
            can_be_opened: true,
            open: false,
            hidden_compartment_open: false,
        };

        let args = FixtureViewArgs {
            knows_items: true,
            knows_hidden: true,
            knows_has_hidden: true,
            knows_can_be_opened: true,
        };

        assert_eq!("steel chest", format!("{}", view(&fixture, &args, true)));
    }
}
