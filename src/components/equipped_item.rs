use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
pub struct EquippedItem<T: Display + Clone + Debug> {
    pub item: T,
    pub hidden: bool,
    pub equipped_location: String,
    pub multiple: bool,
}
