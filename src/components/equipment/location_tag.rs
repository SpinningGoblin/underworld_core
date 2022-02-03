use enum_iterator::IntoEnumIterator;

#[derive(Clone, Debug, IntoEnumIterator, PartialEq)]
pub enum LocationTag {
    Ankle,
    Arm,
    Back,
    Body,
    Equipped,
    Feet,
    Hand,
    Head,
    Hip,
    HipSheath,
    Leg,
    Neck,
    Packed,
    Shoulder,
    Waist,
    Wrist,
}
