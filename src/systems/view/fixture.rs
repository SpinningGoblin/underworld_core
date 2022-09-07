use crate::components::{
    fixtures::{Fixture, FixtureView, FixtureViewArgs},
    items::FixtureItemView,
};

pub fn view(fixture: &Fixture, args: &FixtureViewArgs, knows_all: bool) -> FixtureView {
    let items: Vec<FixtureItemView> = fixture
        .items
        .iter()
        .filter_map(|fixture_item| {
            if fixture_item.is_inside && fixture.open {
                Some(FixtureItemView {
                    item: super::item::view(&fixture_item.item, true, knows_all),
                    is_in_hidden_compartment: Some(false),
                    is_in_hidden_compartment_known: true,
                    is_inside: true,
                })
            } else if fixture_item.is_inside && !fixture.open {
                None
            } else if fixture_item.is_in_hidden_compartment && fixture.hidden_compartment_open {
                Some(FixtureItemView {
                    item: super::item::view(&fixture_item.item, true, knows_all),
                    is_in_hidden_compartment: Some(true),
                    is_in_hidden_compartment_known: true,
                    is_inside: false,
                })
            } else if fixture_item.is_in_hidden_compartment && !fixture.hidden_compartment_open {
                None
            } else {
                Some(FixtureItemView {
                    item: super::item::view(&fixture_item.item, true, knows_all),
                    is_in_hidden_compartment: Some(false),
                    is_in_hidden_compartment_known: true,
                    is_inside: false,
                })
            }
        })
        .collect();

    let (has_hidden, hidden_compartment_open) = if args.knows_has_hidden_compartment || knows_all {
        (
            fixture.has_hidden_compartment,
            fixture.hidden_compartment_open,
        )
    } else {
        (false, false)
    };

    FixtureView {
        id: fixture.id.to_string(),
        name: fixture.name.clone(),
        fixture_type: fixture.fixture_type,
        material: fixture.material,
        size: fixture.size,
        descriptors: fixture.descriptors.clone(),
        items,
        has_hidden_compartment: has_hidden,
        knows_if_hidden_compartment: args.knows_has_hidden_compartment || knows_all,
        open: fixture.open,
        can_be_opened: fixture.can_be_opened,
        hidden_compartment_open,
    }
}
