use super::{
    ancestry::Ancestry, attacks::Attack, attributes::Attributes, class::Class,
    combat_checks::CombatChecks, death_stats::DeathStats, defenses::Defenses,
    knowledges::Knowledges, languages::Language, movement_stats::MovementStats,
    resources::Resources, saves::Saves, skills::Skills, stats::Stats, trades::Trade,
};

#[derive(Debug, Default)]
pub struct PlayerCharacter {
    pub player_name: String,
    pub character_name: String,
    pub level: u8,
    pub class: Class,
    pub ancestry: Ancestry,
    pub attributes: Attributes,
    pub saves: Saves,
    pub skills: Skills,
    pub knowledges: Knowledges,
    pub trades: Vec<Trade>,
    pub languages: Vec<Language>,
    pub stats: Stats,
    pub defenses: Defenses,
    pub combat_checks: CombatChecks,
    pub attacks: Vec<Attack>,
    pub death_stats: DeathStats,
    pub movement_stats: MovementStats,
    pub resources: Resources,
    pub inventory: Vec<String>,
    pub features: Vec<String>,
}
