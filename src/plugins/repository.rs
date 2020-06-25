use crate::open_connection;
use crate::structs;
use crate::structs::NewPlugin;
use mysql::prelude::*;
use structs::Plugin;

pub fn get_last_plugin() -> Result<Plugin, String> {
    let mut conn = open_connection()?;

    match conn.exec_first(
        "SELECT id, name, description, price FROM Plugins ORDER BY id DESC LIMIT 1",
        (),
    ) {
        Ok(result) => match result {
            Some(row) => Ok(Plugin::from_row(row)),
            None => Err("No plugins found".to_string()),
        },
        Err(err) => Err(format!("Get last plugin error: {}", err)),
    }
}

pub fn list_plugins() -> Result<Vec<Plugin>, String> {
    let mut conn = open_connection()?;
    match conn.query_map(
        "SELECT id, name, description, price FROM plugins",
        |(id, name, description, price)| Plugin {
            id,
            name,
            description,
            price,
        },
    ) {
        Ok(value) => Ok(value),
        Err(err) => Err(format!("list_plugins error: {}", err)),
    }
}

pub fn update_plugin(plugin_save: Plugin) -> Result<(), String> {
    let mut conn = open_connection()?;

    match conn.exec_drop(
        "UPDATE plugins SET name = ?, description = ?, price = ? WHERE id = ?",
        (
            plugin_save.name,
            plugin_save.description,
            plugin_save.price,
            plugin_save.id,
        ),
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("update_plugin error: {}", err)),
    }
}

pub fn delete_plugin(id: i32) -> Result<(), String> {
    let mut conn = open_connection()?;

    match conn.exec_drop("DELETE FROM plugins WHERE id = ?", (id,)) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("delete_plugin error: {}", err)),
    }
}

pub fn insert_plugin(plugin_save: NewPlugin) -> Result<(), String> {
    let mut conn = open_connection()?;

    match conn.exec_drop(
        "INSERT INTO plugins (name, description, price) VALUES(?, ?, ?)",
        (plugin_save.name, plugin_save.description, plugin_save.price),
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("insert_plugin error: {}", err)),
    }
}

pub fn get_plugin(id_where: i32) -> std::result::Result<Plugin, String> {
    let mut conn = open_connection()?;

    match conn.exec_first(
        "SELECT id, name, description, price FROM plugins WHERE id = ?",
        (id_where,),
    ) {
        Ok(result) => match result {
            Some(row) => Ok(Plugin::from_row(row)),
            None => Err(format!("Plugin {} not found", id_where)),
        },
        Err(err) => Err(format!("get_plugin error: {}", err)),
    }
}
