use std::sync::Mutex;

use serde::{Serialize, Deserialize};
use tauri::State;

use crate::{models::{cut_disposition::{CutDispositionInput, Rectangle, self, CutDispositionState, ConfigCutDispositionInput, PositionedRectangle}, app_error::AppError}, CutDispositionInputState};

#[tauri::command]
pub async fn get_cut_disposition_input(state: State<'_, CutDispositionInputState>) -> Result<CutDispositionInput, AppError> {
    let a = state.cut_disposition_state.lock();
    match a {
        Ok(cut_disposition_status) => {Ok(cut_disposition_status.get_cut_disposition_input())},
        Err(_) => Err(AppError::new(1, "Erro ao buscar")),
    }
}

#[tauri::command]
pub async fn get_config_cut_disposition_input(state: State<'_, CutDispositionInputState>) -> Result<ConfigCutDispositionInput, AppError> {
    let a = state.cut_disposition_state.lock();
    match a {
        Ok(cut_disposition_status) => {Ok(cut_disposition_status.get_config_cut_disposition_input())},
        Err(_) => Err(AppError::new(1, "Erro ao buscar")),
    }
}

#[tauri::command]
pub async fn set_config_cut_disposition_input(config: ConfigCutDispositionInput, state: State<'_, CutDispositionInputState>) -> Result<ConfigCutDispositionInput, AppError> {
    let cut_disposition_state_result = state.cut_disposition_state.lock();
    match cut_disposition_state_result {
        Ok(mut cut_disposition_state) => {
            cut_disposition_state.spacing = config.spacing;
            cut_disposition_state.max_length = config.max_length;
            cut_disposition_state.defined_length = config.defined_length;
            cut_disposition_state.defined_width = config.defined_width;
            Ok(cut_disposition_state.get_config_cut_disposition_input())
        },
        Err(_) => Err(AppError::new(1, "Erro ao configurarar cortes")),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum RectangleType {
    Piece(Rectangle),
    Showcase(Rectangle),
    ProhibitedArea(PositionedRectangle)
}

#[tauri::command]
pub async fn create_piece(piece: RectangleType, state: State<'_, CutDispositionInputState>) -> Result<(), AppError> {
    let cut_disposition_state_result = state.cut_disposition_state.lock();
    match cut_disposition_state_result {
        Ok(mut cut_disposition_state) => {
            match piece {
                RectangleType::Piece(main_piece) => {
                    cut_disposition_state.add_piece(&main_piece);
                },
                RectangleType::Showcase(showcase) => {
                    cut_disposition_state.add_showcase(&showcase);
                },
                RectangleType::ProhibitedArea(prohibited_area) => {
                    cut_disposition_state.add_prohibited_area(&prohibited_area);
                },
            };
            Ok(())
        },
        Err(_) => Err(AppError::new(1, "Erro ao configurarar cortes")),
    }
}