#[derive(Clone, Debug)]
pub struct Name(pub String);

#[derive(Clone, Debug)]
pub struct Player {
    pub character: Character,
}

#[derive(Clone, Debug)]
pub struct NonPlayer {
    pub character: Character,
}

#[derive(Clone, Debug)]
pub struct Character {
    pub name: Option<Name>,
}
