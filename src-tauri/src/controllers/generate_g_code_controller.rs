use std::{time::Duration, path};

use tauri::{State, api::path::home_dir};
use tokio::time::sleep;

use crate::{CutDispositionInputState, models::{app_error::AppError, cut_disposition::{CutDispositionInput, CutDispositionOutput}}, services::{cut_disposition_service::organize_disposition, cutting_lines_service::define_cutting_lines, gcode_service::generate_gcode_file, file_service::{FileError, GENERATED_FILES_FOLDER, GCODE_FOLDER}}};

#[tauri::command]
pub async fn generate_g_code(file_name: String, pull_textile: bool, state: State<'_, CutDispositionInputState>) -> Result<String, AppError> {
    if file_name.trim() == "" {
        return Err(AppError::new(1, format!("Nome inválido: {}", file_name).as_str()));
    }

    sleep(Duration::from_millis(1)).await;
    
    let cut_disposition_output = get_cut_disposition_output(state)?;

    let mut rectangle_list = cut_disposition_output.positioned_rectangles_list.clone();
    rectangle_list.append(&mut cut_disposition_output.showcase_rectangles_located_list.clone());

    let cutting_lines = define_cutting_lines(rectangle_list, Some(cut_disposition_output.defined_width));

    let mut textile_length_to_pull = None;
    if pull_textile {
        if let Ok(lenght) = u32::try_from(cut_disposition_output.length_used) {
            textile_length_to_pull = Some(lenght);
        } else {
            return Err(AppError::new(2, "Erro ao converter comprimento do tecido"));
        } 
    }


    let mut home_path = "gcode".to_string();
    if let Some(home_path_buf) = home_dir() {
        if let Some(home_str) = home_path_buf.to_str() {
            home_path = format!("{}{}{}{}{}", 
            home_str, 
            path::MAIN_SEPARATOR_STR,
            GENERATED_FILES_FOLDER,
            path::MAIN_SEPARATOR_STR,
            GCODE_FOLDER);
        }
    }

    generate_gcode_file(
        cutting_lines.horizontal_lines, 
        cutting_lines.vertical_lines, 
        textile_length_to_pull, 
        home_path.as_str(), 
        &file_name).await.map_err(map_file_error_to_app_error)
}

fn map_file_error_to_app_error(error: FileError) -> AppError {
    match error {
        FileError::FailedToOpenFile { path } => AppError::new(1, &format!("Falha ao abrir localizado em: {}", path)),
        FileError::FailedToReadFile { path } => AppError::new(1, &format!("Falha ao ler conteúdo arquivo localizado em: {}", path)),
        FileError::FailedToWriteFile { path } => AppError::new(1, &format!("Falha ao escrever em arquivo localizado em: {}", path)),
    }
}

fn get_cut_disposition_output(state: State<'_, CutDispositionInputState>) -> Result<CutDispositionOutput, AppError>{
    let state_result = state.cut_disposition_state.lock();
    match state_result {
        Ok(cut_disposition_state) => {
            if cut_disposition_state.spacing.is_some_and(|space| space <= 0) {
                return Err(AppError::new(1, "Erro ao organizar disposição: espaçamento deve ser maior que zero ou inexistente"));
            }
            if cut_disposition_state.defined_length.is_some_and(|space| space <= 0) {
                return Err(AppError::new(1, "Erro ao organizar disposição: comprimento definido deve ser maior que a zero ou inexistente"));
            }
            if cut_disposition_state.max_length <= 0 {
                return Err(AppError::new(1, "Erro ao organizar disposição: comprimento máximo deve ser maior que a zero"));
            }
            if cut_disposition_state.defined_width <= 0 {
                return Err(AppError::new(1, "Erro ao organizar disposição: largura máxima deve ser maior que zero"));
            }

            Ok(organize_disposition(&CutDispositionInput {
                rectangles_list: cut_disposition_state.rectangles_list.clone(),
                prohibited_area_list: cut_disposition_state.prohibited_area_list.clone(),
                showcase: cut_disposition_state.showcase.clone(),
                spacing: cut_disposition_state.spacing,
                max_length: cut_disposition_state.max_length,
                defined_length: cut_disposition_state.defined_length,
                defined_width: cut_disposition_state.defined_width
            }))

        },
        Err(_) => Err(AppError::new(1, "Erro ao organizar disposição")),
    }
}