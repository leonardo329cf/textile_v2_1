// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod db;
use std::sync::{Arc, Mutex};

use db::db_connection::DbConnection;
use models::cut_disposition::CutDispositionState;

use crate::controllers::{about_controller::get_about, cut_disposition_controller::{get_cut_disposition_input, set_config_cut_disposition_input, get_config_cut_disposition_input, create_piece}};
use controllers::fabric_controller::{get_fabric, get_all_fabric, delete_fabric, create_fabric, update_fabric};

mod controllers;
mod services;
mod models;

pub struct CutDispositionInputState {
    pub cut_disposition_state: Arc<Mutex<CutDispositionState>>
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let db_connection = DbConnection::new().await.expect("Error initializing db");
    tauri::Builder::default()
        .manage(db_connection)
        .manage(CutDispositionInputState {
            cut_disposition_state: Arc::new(Mutex::new(CutDispositionState::new()))
        })
        .invoke_handler(tauri::generate_handler![
            get_about,
            get_fabric, get_all_fabric, delete_fabric, create_fabric, update_fabric,
            get_cut_disposition_input, set_config_cut_disposition_input, get_config_cut_disposition_input,
            create_piece])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
