#[derive(Debug, Default)]
pub struct Stats {
    pub stamina: Stat,
    pub mana: Stat,
    pub hit_points: Stat,
    pub temporary_hit_points: u8,
}

#[derive(Debug, Default)]
pub struct Stat {
    max: u8,
    current: u8,
}
