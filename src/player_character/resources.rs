#[derive(Debug, Default)]
pub struct Resources {
    rest_points: Resource,
    grit_points: Resource,
    custom_resources: Vec<CustomResource>,
}

#[derive(Debug, Default)]
pub struct Resource {
    max: u8,
    current: u8,
}

#[derive(Debug, Default)]
pub struct CustomResource {
    name: String,
    max: u8,
    current: u8,
}
