// Skill, Knoledge, Trade
#[derive(Debug, Default)]
pub enum MasteryLevel {
    #[default]
    None,
    Novice,
    Adept,
    Expert,
    Master,
    GrandMaster,
}

// Language
#[derive(Debug, Default)]
pub enum LanguageMasteryLevel {
    #[default]
    None,
    Limited,
    Fluent,
}
