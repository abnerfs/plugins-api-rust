#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

extern crate mysql;

#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;

mod database;

mod structs;
use structs::Plugin;

use rocket_contrib::json::Json;

// use mysql::*;
use mysql::prelude::*;

#[path = "plugins/repository.rs"]
mod plugins_repository;

lazy_static! {
    static ref POOL: mysql::Pool = database::create_pool();
}

#[get("/api/plugins")]
fn list_plugins() -> Json<Vec<Plugin>> {
    let mut conn = POOL.get_conn().expect("Failed to open connection");

    let selected_plugins = conn
        .query_map(
            "SELECT id, name, description, price FROM plugins",
            |(id, name, description, price)| Plugin {
                id,
                name,
                description,
                price,
            },
        )
        .expect("Failed to get plugins");

    Json(selected_plugins)
}

#[post("/api/plugins", format = "application/json", data = "<plugin>")]
fn create_plugin(plugin: Json<Plugin>) -> Json<Plugin> {
    let plugin_save = plugin.into_inner();

    let mut conn = POOL.get_conn().expect("Failed to open connection");

    match conn.exec_drop(
        "INSERT INTO plugins (name, description, price) VALUES(?, ?, ?)",
        (plugin_save.name, plugin_save.description, plugin_save.price),
    )
    {
        Ok(_) => {
            match plugins_repository::get_plugin(1, conn) {
                Ok(saved) => Json(saved),
                Err(err) => Json(Plugin {
                    description: format!("ERRO GET {}", err),
                    id: 0,
                    name: "".to_string(),
                    price: 0.0
                })
            }
        },
        Err(err) => Json(Plugin {
            description: format!("ERRO insert {}", err),
            id: 0,
            name: "".to_string(),
            price: 0.0
        })
    }
    
}

fn main() {
    dotenv().ok();
    rocket::ignite().mount("/", routes![list_plugins, create_plugin]).launch();
}
