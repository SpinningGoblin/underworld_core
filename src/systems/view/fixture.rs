use crate::components::{
    fixtures::fixture::{Fixture, FixtureView},
    items::item::ItemView,
};

pub fn look_at(
    fixture: &Fixture,
    knows_contained: bool,
    knows_hidden: bool,
    knows_all: bool,
) -> FixtureView {
    let contained_items: Vec<ItemView> = if knows_contained || knows_all {
        fixture
            .contained_items
            .iter()
            .map(|item| super::item::look_at(item, knows_contained, knows_all))
            .collect()
    } else {
        Vec::new()
    };

    let hidden_items: Vec<ItemView> = if knows_hidden || knows_all {
        fixture
            .contained_items
            .iter()
            .map(|item| super::item::look_at(item, knows_hidden, knows_all))
            .collect()
    } else {
        Vec::new()
    };

    let has_hidden = if knows_hidden || knows_all {
        fixture.has_hidden_compartment
    } else {
        false
    };

    FixtureView {
        identifier: super::identifier::to_view(&fixture.identifier, true),
        fixture_type: fixture.fixture_type.clone(),
        material: fixture.material.clone(),
        size: fixture.size.clone(),
        descriptors: fixture.descriptors.clone(),
        contained_items,
        knows_contained_items: knows_contained || knows_all,
        hidden_compartment_items: hidden_items,
        has_hidden_compartment: has_hidden,
        knows_hidden_compartment_items: knows_hidden || knows_all,
        knows_if_hidden_compartment: knows_hidden || knows_all,
    }
}
