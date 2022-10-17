#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Action {
    AttackNpc(super::AttackNpc),
    CastSpellOnNpc(super::CastSpellOnNpc),
    CastSpellOnPlayer(super::CastSpellOnPlayer),
    ExitRoom(super::ExitRoom),
    InspectFixture(super::InspectFixture),
    InspectNpc(super::InspectNpc),
    LookAtFixture(super::LookAtFixture),
    LookAtNpc(super::LookAtNpc),
    LootFixture(super::LootFixture),
    LootNpc(super::LootNpc),
    MovePlayerItem(super::MovePlayerItem),
    OpenFixture(super::OpenFixture),
    OpenFixtureHiddenCompartment(super::OpenFixtureHiddenCompartment),
    PickUpItem(super::PickUpItem),
    SellPlayerItem(super::SellPlayerItem),
    UseItemOnPlayer(super::UseItemOnPlayer),
    ThrowItemAtNpc(super::ThrowItemAtNpc),
}
