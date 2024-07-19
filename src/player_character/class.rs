#[derive(Debug, Default)]
pub enum Class {
    #[default]
    Unkown,
    Barbarian(),
    Bard(),
    Cleric(),
    Commander(),
    Druid(),
    Fighter(),
    Monk(),
    Ranger(),
    Rogue(),
    Sorcerer(),
    Spellblade(),
    Warlock(),
    Wizard(),
}
