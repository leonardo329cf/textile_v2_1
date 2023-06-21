// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
use std::sync::{Arc, Mutex};

use db::db_connection::DbConnection;
use models::cut_disposition::CutDispositionState;

use controllers::{
    about_controller::get_about,
    cut_disposition_controller::{
        get_config_cut_disposition_input, get_cut_disposition_input,
        set_config_cut_disposition_input, create_piece, get_piece, 
        get_showcase, get_prohibited_area, edit_piece, delete_piece, 
        delete_showcase, delete_prohibited_area
    },
    cutting_table_controller::{
        create_cutting_table, delete_cutting_table, get_all_cutting_table, get_cutting_table,
        update_cutting_table,
    },
    fabric_controller::{create_fabric, delete_fabric, get_all_fabric, get_fabric, update_fabric},
};

use crate::controllers::cut_disposition_controller::organize_cut_disposition;

mod controllers;
mod models;
mod services;

pub struct CutDispositionInputState {
    pub cut_disposition_state: Arc<Mutex<CutDispositionState>>,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let db_connection = DbConnection::new().await.expect("Error initializing db");
    tauri::Builder::default()
        .manage(db_connection)
        .manage(CutDispositionInputState {
            cut_disposition_state: Arc::new(Mutex::new(CutDispositionState::new())),
        })
        .invoke_handler(tauri::generate_handler![
            get_about,
            get_fabric, get_all_fabric, delete_fabric, create_fabric, update_fabric,
            get_cutting_table, get_all_cutting_table, delete_cutting_table, create_cutting_table, update_cutting_table,
            get_cut_disposition_input, set_config_cut_disposition_input, get_config_cut_disposition_input,
            create_piece, get_piece, get_showcase, get_prohibited_area, edit_piece, 
            delete_piece, delete_showcase, delete_prohibited_area,
            organize_cut_disposition])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
