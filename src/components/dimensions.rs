use super::species::Species;

#[derive(Clone, Debug)]
pub struct Dimensions {
    pub height: f32,
    pub width: f32,
}

impl Dimensions {
    pub fn describe_height_for_species(&self, species: &Species) -> String {
        match species {
            &Species::Goblin => self.goblin_height(),
            _ => "".to_string(),
        }
    }

    fn goblin_height(&self) -> String {
        if self.is_taller_than(1.2) {
            "tall".to_string()
        } else if self.is_shorter_than(0.6) {
            "short".to_string()
        } else {
            "".to_string()
        }
    }

    fn is_taller_than(&self, height: f32) -> bool {
        self.height > height
    }

    fn is_shorter_than(&self, height: f32) -> bool {
        self.height < height
    }
}
