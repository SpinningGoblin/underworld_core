#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::descriptor_position::DescriptorPosition;

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum NpcPositionDescriptor {
    IsGlaringAtYou,
    IsGlaringAtYouFromNearby,
    InCornerStands,
    IsStandingAround,
    IsLeaningAgainstTheTable,
    StandsOnTheTable,
    IsStandingOnTheTable,
    IsSittingInAChair,
    IsStandingInABarrel,
    IsSleepingInTheBed,
    IsSleepingInACot,
    IsSleepingInSleepingRoll,
    IsLeaningOnACrate,
    IsLookingAtTheWeaponRack,
    IsCrouchedOverChest,
    IsRummagingThroughAChest,
    IsCrouchedInTheCenterOfRoom,
    IsSittingAndDozingInCenterOfRoom,
    SittingInAChairIs,
    IsLyingInPoolBlood,
}

impl NpcPositionDescriptor {
    fn descriptor_position(&self) -> DescriptorPosition {
        match *self {
            NpcPositionDescriptor::InCornerStands => DescriptorPosition::Pre,
            NpcPositionDescriptor::IsGlaringAtYou => DescriptorPosition::Post,
            NpcPositionDescriptor::IsGlaringAtYouFromNearby => DescriptorPosition::Post,
            NpcPositionDescriptor::IsStandingAround => DescriptorPosition::Post,
            NpcPositionDescriptor::IsLeaningAgainstTheTable => DescriptorPosition::Post,
            NpcPositionDescriptor::StandsOnTheTable => DescriptorPosition::Post,
            NpcPositionDescriptor::IsStandingOnTheTable => DescriptorPosition::Post,
            NpcPositionDescriptor::IsSittingInAChair => DescriptorPosition::Post,
            NpcPositionDescriptor::IsStandingInABarrel => DescriptorPosition::Post,
            NpcPositionDescriptor::IsSleepingInTheBed => DescriptorPosition::Post,
            NpcPositionDescriptor::IsSleepingInACot => DescriptorPosition::Post,
            NpcPositionDescriptor::IsSleepingInSleepingRoll => DescriptorPosition::Post,
            NpcPositionDescriptor::IsLeaningOnACrate => DescriptorPosition::Post,
            NpcPositionDescriptor::IsLookingAtTheWeaponRack => DescriptorPosition::Post,
            NpcPositionDescriptor::IsCrouchedOverChest => DescriptorPosition::Post,
            NpcPositionDescriptor::IsCrouchedInTheCenterOfRoom => DescriptorPosition::Post,
            NpcPositionDescriptor::IsSittingAndDozingInCenterOfRoom => DescriptorPosition::Post,
            NpcPositionDescriptor::SittingInAChairIs => DescriptorPosition::Pre,
            NpcPositionDescriptor::IsRummagingThroughAChest => DescriptorPosition::Post,
            NpcPositionDescriptor::IsLyingInPoolBlood => DescriptorPosition::Post,
        }
    }

    pub fn is_pre(&self) -> bool {
        self.descriptor_position() == DescriptorPosition::Pre
    }

    pub fn is_post(&self) -> bool {
        self.descriptor_position() == DescriptorPosition::Post
    }
}
