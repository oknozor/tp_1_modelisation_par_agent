#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub x: f32,
    pub y: f32,
    pub cell_size: f32,
    pub agents : u32,
}

