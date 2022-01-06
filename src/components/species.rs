use crate::describable::Describable;

#[derive(Clone, Debug)]
pub enum Species {
    BugBear,
    Elf,
    Goblin,
    Human,
    Kobold,
    Ogre,
    Orc,
    Unknown,
}

impl Describable for Species {
    fn describe(&self) -> String {
        match *self {
            Self::BugBear => "Bug Bear".to_string(),
            Self::Elf => "Elf".to_string(),
            Self::Goblin => "Goblin".to_string(),
            Self::Human => "Human".to_string(),
            Self::Kobold => "Kobold".to_string(),
            Self::Ogre => "Ogre".to_string(),
            Self::Orc => "Orc".to_string(),
            _ => "Mysterious Entity".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{components::species::Species, describable::Describable};

    #[test]
    fn describe_when_bugbear() {
        assert_eq!("Bug Bear", Species::BugBear.describe());
    }

    #[test]
    fn describe_when_elf() {
        assert_eq!("Elf", Species::Elf.describe());
    }

    #[test]
    fn describe_when_goblin() {
        assert_eq!("Goblin", Species::Goblin.describe());
    }

    #[test]
    fn describe_when_kobold() {
        assert_eq!("Kobold", Species::Kobold.describe());
    }

    #[test]
    fn describe_when_human() {
        assert_eq!("Human", Species::Human.describe());
    }

    #[test]
    fn describe_when_orc() {
        assert_eq!("Orc", Species::Orc.describe());
    }

    #[test]
    fn describe_when_unknown() {
        assert_eq!("Mysterious Entity", Species::Unknown.describe());
    }
}
