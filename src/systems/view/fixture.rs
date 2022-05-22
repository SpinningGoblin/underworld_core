use crate::components::{
    fixtures::fixture::{Fixture, FixtureView, FixtureViewArgs},
    items::fixture_item::FixtureItemView,
};

pub fn view(fixture: &Fixture, args: &FixtureViewArgs, knows_all: bool) -> FixtureView {
    let items: Vec<FixtureItemView> = if args.knows_items || knows_all {
        fixture
            .items
            .iter()
            .filter(|fixture_item| {
                if fixture_item.is_hidden {
                    args.knows_hidden || knows_all
                } else {
                    true
                }
            })
            .map(|fixture_item| {
                let is_hidden = if fixture_item.is_hidden {
                    args.knows_hidden || knows_all
                } else {
                    false
                };
                FixtureItemView {
                    item: super::item::view(&fixture_item.item, args.knows_items, knows_all),
                    is_hidden,
                    is_hidden_known: args.knows_hidden,
                }
            })
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
        identifier: super::identifier::view(&fixture.identifier, true),
        fixture_type: fixture.fixture_type.clone(),
        material: fixture.material.clone(),
        size: fixture.size.clone(),
        descriptors: fixture.descriptors.clone(),
        items,
        knows_contained_items: args.knows_items || knows_all,
        has_hidden_compartment: has_hidden,
        knows_hidden_compartment_items: args.knows_hidden || knows_all,
        knows_if_hidden_compartment: args.knows_has_hidden || knows_all,
        open,
        can_be_opened,
        knows_if_can_be_opened: args.knows_can_be_opened || knows_all,
        hidden_compartment_open,
    }
}
