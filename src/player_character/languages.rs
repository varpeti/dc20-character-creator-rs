use super::mastery_level::LanguageMasteryLevel;

#[derive(Debug)]
pub struct Language {
    pub name: LanguageName,
    pub value: i32,
    pub mastery_level: LanguageMasteryLevel,
}

#[derive(Debug)]
pub enum LanguageName {
    Mortal(Mortal),
    Exotic(Exotic),
    Divine(Divine),
    Outer(Outer),
    Special(String),
}

#[derive(Debug)]
pub enum Mortal {
    Common,
    Human,
    Dwarven,
    Elvish,
    Gnomish,
    Halfling,
    Orchish,
}

#[derive(Debug)]
pub enum Exotic {
    Giant,
    Draconic,
    Fey,
    Elemental,
}

#[derive(Debug)]
pub enum Divine {
    Celestial,
    Fiend,
}

#[derive(Debug)]
pub enum Outer {
    DeepSpeech,
}
