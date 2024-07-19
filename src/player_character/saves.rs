#[derive(Debug, Default)]
pub struct Saves {
    pub might: Save,
    pub agility: Save,
    pub charisma: Save,
    pub intelligence: Save,
}

#[derive(Debug, Default)]
pub struct Save {
    pub value: i32,
    pub has_mastery: bool,
}
