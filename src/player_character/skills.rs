use super::mastery_level::MasteryLevel;

#[derive(Debug, Default)]
pub struct Skills {
    pub prime: Prime,
    pub might: Might,
    pub agility: Agility,
    pub charisma: Charisma,
    pub intelligence: Intelligence,
}

#[derive(Debug, Default)]
pub struct Prime {
    pub awerness: Skill,
}

#[derive(Debug, Default)]
pub struct Might {
    pub athletics: Skill,
    pub intimidation: Skill,
}

#[derive(Debug, Default)]
pub struct Agility {
    pub acrobatics: Skill,
    pub trickery: Skill,
    pub stealth: Skill,
}

#[derive(Debug, Default)]
pub struct Charisma {
    pub animal: Skill,
    pub influence: Skill,
    pub insight: Skill,
}

#[derive(Debug, Default)]
pub struct Intelligence {
    pub investigation: Skill,
    pub medicine: Skill,
    pub survival: Skill,
}

#[derive(Debug, Default)]
pub struct Skill {
    pub value: i32,
    pub mastery_level: MasteryLevel,
}
