
#[derive(Serialize, Deserialize)]
pub struct Plugin {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f32
}