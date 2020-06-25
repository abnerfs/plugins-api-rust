#[derive(Serialize, Deserialize)]
pub struct Plugin {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f32,
}

#[derive(Serialize, Deserialize)]
pub struct NewPlugin {
    pub name: String,
    pub description: String,
    pub price: f32,
}

use mysql::*;

impl Plugin {
    pub fn from_row(row: Row) -> Plugin {
        let (id, name, description, price): (i32, String, String, f32) = from_row(row);
        Plugin {
            id: id,
            name: name,
            description: description,
            price: price,
        }
    }
}
