// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod db;
use db::db_connection::DbConnection;

use crate::controllers::about_controller::get_about;
use controllers::fabric_controller::{get_fabric, get_all_fabric, delete_fabric, create_fabric, update_fabric};

mod controllers;
mod services;
mod models;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let db_connection = DbConnection::new().await.expect("Error initializing db");
    tauri::Builder::default()
        .manage(db_connection)
        .invoke_handler(tauri::generate_handler![
            get_about,
            get_fabric, get_all_fabric, delete_fabric, create_fabric, update_fabric
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
