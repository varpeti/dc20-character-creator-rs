#[derive(Debug, Default)]
pub struct Attributes {
    pub prime: Attribute,
    pub might: Attribute,
    pub agility: Attribute,
    pub charisma: Attribute,
    pub intelligence: Attribute,
}

#[derive(Debug, Default)]
pub struct Attribute {
    pub value: i32,
}
