#[derive(Debug)]
pub struct Attack {
    name: String,
    damage: i32,
    damage_type: DamageType,
}

#[derive(Debug)]
pub enum DamageType {
    Slashing,
    Piercing,
    Bludgeoning,
    Fire,
    Lightning,
    Cold,
    Sonic,
    Poison,
    Corrosion,
    Radiant,
    Umbral,
    Psychic,
    True,
}
