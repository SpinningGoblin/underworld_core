use crate::components::{
    fixtures::fixture::{Fixture, FixtureView, FixtureViewArgs},
    items::item::ItemView,
};

pub fn look_at(fixture: &Fixture, args: &FixtureViewArgs, knows_all: bool) -> FixtureView {
    let contained_items: Vec<ItemView> = if args.knows_items || knows_all {
        fixture
            .contained_items
            .iter()
            .map(|item| super::item::look_at(item, args.knows_items, knows_all))
            .collect()
    } else {
        Vec::new()
    };

    let hidden_items: Vec<ItemView> = if args.knows_hidden || knows_all {
        fixture
            .contained_items
            .iter()
            .map(|item| super::item::look_at(item, args.knows_has_hidden, knows_all))
            .collect()
    } else {
        Vec::new()
    };

    let (has_hidden, hidden_compartment_open) = if args.knows_has_hidden || knows_all {
        (
            fixture.has_hidden_compartment,
            fixture.hidden_compartment_open,
        )
    } else {
        (false, false)
    };

    let (open, can_be_opened) = if args.knows_can_be_opened || knows_all {
        (fixture.open, fixture.can_be_opened)
    } else {
        (false, false)
    };

    FixtureView {
        identifier: super::identifier::to_view(&fixture.identifier, true),
        fixture_type: fixture.fixture_type.clone(),
        material: fixture.material.clone(),
        size: fixture.size.clone(),
        descriptors: fixture.descriptors.clone(),
        contained_items,
        knows_contained_items: args.knows_items || knows_all,
        hidden_compartment_items: hidden_items,
        has_hidden_compartment: has_hidden,
        knows_hidden_compartment_items: args.knows_hidden || knows_all,
        knows_if_hidden_compartment: args.knows_has_hidden || knows_all,
        open,
        can_be_opened,
        knows_if_can_be_opened: args.knows_can_be_opened || knows_all,
        hidden_compartment_open,
    }
}
