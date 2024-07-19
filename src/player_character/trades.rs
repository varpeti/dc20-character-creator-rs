use super::mastery_level::MasteryLevel;

#[derive(Debug)]
pub struct Trade {
    pub name: TradeName,
    pub value: i32,
    pub mastery_level: MasteryLevel,
}

#[derive(Debug)]
pub enum TradeName {
    Artistry(Artistry),
    Crafting(Crafting),
    Services(Services),
    Subterfuge(Subterfuge),
}

#[derive(Debug)]
pub enum Artistry {
    Illustartion,
    Musician,
    Theatre,
}

#[derive(Debug)]
pub enum Crafting {
    Alchemy,
    BlackSmithing,
    Glassblowing,
    Herbalism,
    Jeweler,
    Leatherworking,
    Sculpting,
    Tinkering,
    Weaving,
}

#[derive(Debug)]
pub enum Services {
    Brewing,
    Carpentry,
    Cartography,
    Cooking,
    Masonry,
    Vehicles,
}

#[derive(Debug)]
pub enum Subterfuge {
    Cryptography,
    Disguise,
    Gaming,
    LockPicking,
}
