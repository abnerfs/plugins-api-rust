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
use rocket::http::Status;
use rocket::response;
use rocket::response::status;
use rocket::response::Responder;
use rocket::Request;

mod database;

mod structs;
use structs::NewPlugin;
use structs::Plugin;

use rocket_contrib::json::Json;

#[path = "plugins/repository.rs"]
mod plugins_repository;

lazy_static! {
    pub static ref POOL: mysql::Pool = database::create_pool();
}

pub fn open_connection() -> Result<mysql::PooledConn, String> {
    match POOL.get_conn() {
        Ok(cn) => Ok(cn),
        Err(_) => Err("Error opening connection".to_string()),
    }
}

#[derive(Serialize, Deserialize)]
pub struct CustomError {
    pub message: String,
}

impl CustomError {
    fn new(message: String) -> CustomError {
        CustomError { message: message }
    }
}

pub enum PluginResponse<R> {
    Ok(R),
    Err(String),
}

impl<'r> Responder<'r> for PluginResponse<Vec<Plugin>> {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        match self {
            PluginResponse::Ok(value) => status::Custom(Status::Ok, Json(value)).respond_to(req),
            PluginResponse::Err(x) => {
                status::Custom(Status::InternalServerError, Json(CustomError::new(x)))
                    .respond_to(req)
            }
        }
    }
}

#[get("/api/plugins")]
fn list_plugins() -> PluginResponse<Vec<Plugin>> {
    match plugins_repository::list_plugins() {
        Ok(values) => PluginResponse::Ok(values),
        Err(x) => PluginResponse::Err(x),
    }
}

impl<'r> Responder<'r> for PluginResponse<Plugin> {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        match self {
            PluginResponse::Ok(value) => status::Custom(Status::Ok, Json(value)).respond_to(req),
            PluginResponse::Err(x) => {
                status::Custom(Status::InternalServerError, Json(CustomError::new(x)))
                    .respond_to(req)
            }
        }
    }
}

#[put("/api/plugins/<id>", format = "application/json", data = "<plugin>")]
fn update_plugin(id: i32, plugin: Json<Plugin>) -> PluginResponse<Plugin> {
    let mut plugin_save = plugin.into_inner();
    plugin_save.id = id;

    match plugins_repository::update_plugin(plugin_save) {
        Ok(_) => match plugins_repository::get_plugin(id) {
            Ok(saved) => PluginResponse::Ok(saved),
            Err(err) => PluginResponse::Err(err),
        },
        Err(_) => PluginResponse::Err("Error trying to insert plugin".to_string()),
    }
}

#[delete("/api/plugins/<id>")]
fn delete_plugin(id: i32) -> &'static str {
    match plugins_repository::delete_plugin(id) {
        Ok(_) => "Done",
        Err(_) => "Error trying to insert plugin",
    }
}

#[post("/api/plugins", format = "application/json", data = "<plugin>")]
fn create_plugin(plugin: Json<NewPlugin>) -> PluginResponse<Plugin> {
    let plugin_save = plugin.into_inner();

    match plugins_repository::insert_plugin(plugin_save) {
        Ok(_) => match plugins_repository::get_last_plugin() {
            Ok(saved) => PluginResponse::Ok(saved),
            Err(err) => PluginResponse::Err(err),
        },
        Err(_) => PluginResponse::Err("Error trying to insert plugin".to_string()),
    }
}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .mount("/", routes![list_plugins, create_plugin, update_plugin, delete_plugin])
        .launch();
}
