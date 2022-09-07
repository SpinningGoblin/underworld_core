#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum NpcPositionDescriptor {
    InCornerStands,
    IsCrouchedInTheCenterOfRoom,
    IsCrouchedOverChest,
    IsGlaringAtYou,
    IsGlaringAtYouFromNearby,
    IsLeaningAgainstTheTable,
    IsLeaningOnACrate,
    IsLookingAtTheWeaponRack,
    IsLyingInPoolBlood,
    IsRummagingThroughAChest,
    IsSittingInAChair,
    IsStandingAround,
    IsStandingOnTheTable,
    IsStandingInABarrel,
    SittingInAChairIs,
    StandsOnTheTable,
}
