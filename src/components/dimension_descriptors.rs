use std::ops::Range;

pub trait HeightDescriptor {
    fn height_range(&self) -> Range<f32>;
    fn bigger_text(&self) -> String;
    fn smaller_text(&self) -> String;
    fn average_text(&self) -> String;
}

pub trait WidthDescriptor {
    fn width_range(&self) -> Range<f32>;
    fn bigger_text(&self) -> String;
    fn smaller_text(&self) -> String;
    fn average_text(&self) -> String;
}
