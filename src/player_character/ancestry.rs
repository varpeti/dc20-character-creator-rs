#[derive(Debug, Default)]
pub struct Ancestry {
    parents: [Parent; 2],
    traits: Vec<AncestryTrait>,
}

#[derive(Debug, Default)]
pub enum Parent {
    #[default]
    Unkown,
    Angleborn,
    Beastborn,
    Dragonborn,
    Dwarf,
    Elf,
    Fiendborn,
    Giantborn,
    Gnome,
    Halfling,
    Human,
    Orc,
}

#[derive(Debug, Default)]
pub struct AncestryTrait {
    name: String, // TODO enum
    cost: i32,
}
