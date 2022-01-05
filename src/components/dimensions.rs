#[derive(Clone, Debug)]
pub struct Dimensions {
    pub height: i32,
    pub width: i32,
    pub units: DimensionUnits,
}

#[derive(Clone, Debug)]
pub enum DimensionUnits {
    METER,
    CENTIMETER,
}
