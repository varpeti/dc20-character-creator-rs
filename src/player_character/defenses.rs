#[derive(Debug, Default)]
pub struct Defenses {
    pub physical: Defense,
    pub mystical: Defense,
}

#[derive(Debug, Default)]
pub struct Defense {
    value: u8,
    reduction: u8,
}
