use mysql::*;
use mysql::prelude::*;

use crate::structs;
use structs::Plugin;

pub fn get_plugin(id_where: i32, mut conn: mysql::PooledConn) -> Result<Plugin> {

    let row: Option<Row> = 
        conn.exec_first("SELECT id, name, description, price FROM plugins WHERE id = ?", (id_where, ))?;
            

    let (id, name, description, price): (i32, String, String, f32) = from_row(row.unwrap());
    Ok(
        Plugin {
            id: id,
            name: name,
            description: description,
            price: price
        }
    )

    // let plugin = conn
    //     .query_map(
    //         "SELECT id, name, description, price FROM plugins WHERE id = ?",
    //         |(id, name, description, price)| Plugin {
    //             id,
    //             name,
    //             description,
    //             price,
    //         },
    //     )
    //     .expect("Failed to get plugin");

    // if plugin.len() == 0 {
    //     None
    // } else {
    //     Some(plugin[0])
    // }
}
