use super::mastery_level::MasteryLevel;

#[derive(Debug, Default)]
pub struct Knowledges {
    arcana: Knowledge,
    history: Knowledge,
    nature: Knowledge,
    occultism: Knowledge,
    religion: Knowledge,
    player_defined_a: PlayerDefinedKnowledge,
    player_defined_b: PlayerDefinedKnowledge,
}

#[derive(Debug, Default)]
pub struct Knowledge {
    pub value: i32,
    pub mastery_level: MasteryLevel,
}

#[derive(Debug, Default)]
pub struct PlayerDefinedKnowledge {
    pub name: String,
    pub value: i32,
    pub mastery_level: MasteryLevel,
}
