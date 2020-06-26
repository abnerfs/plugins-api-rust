#[derive(Serialize, Deserialize)]
pub struct Plugin {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub link_git: String
}

#[derive(Serialize, Deserialize)]
pub struct NewPlugin {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub link_git: String,
    pub price: f32,
}

use mysql::*;



impl Plugin {
    pub fn from_row(row: Row) -> Plugin {
        let (id, name, description, price, link_git): (i32, String, String, f32, String) = from_row(row);
        Plugin {
            id: id,
            name: name,
            description: description,
            price: price,
            link_git: link_git
        }
    }
}
