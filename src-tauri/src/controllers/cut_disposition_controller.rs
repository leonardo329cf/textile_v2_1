use serde::{Serialize, Deserialize};
use tauri::State;

use crate::{models::{cut_disposition::{CutDispositionInput, Rectangle, ConfigCutDispositionInput, PositionedRectangle, CutDispositionOutput}, app_error::AppError}, CutDispositionInputState, services::cut_disposition_service::organize_disposition};

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
            if config.spacing.is_some_and(|space| space <= 0) {
                return Err(AppError::new(1, "Erro ao configurar disposição: espaçamento deve ser maior que zero ou inexistente"));
            }
            if config.defined_length.is_some_and(|space| space <= 0) {
                return Err(AppError::new(1, "Erro ao configurar disposição: comprimento definido deve ser maior que zero ou inexistente"));
            }
            if config.max_length <= 0 {
                return Err(AppError::new(1, "Erro ao configurar disposição: comprimento máximo deve ser maior que zero"));
            }
            if config.defined_width <= 0 {
                return Err(AppError::new(1, "Erro ao configurar disposição: largura máxima deve ser maior que zero"));
            }

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
                    if !main_piece.is_valid() {
                        return Err(AppError::new(1, "Erro ao adicionar Peça: dimensões devem ser maiores que zero"));
                    }
                    cut_disposition_state.add_piece(&main_piece);
                },
                RectangleType::Showcase(showcase) => {
                    if !showcase.is_valid() {
                        return Err(AppError::new(1, "Erro ao  adicionar Mostruário: dimensões devem ser maiores que zero"));
                    }
                    cut_disposition_state.add_showcase(&showcase);
                },
                RectangleType::ProhibitedArea(prohibited_area) => {
                    if !prohibited_area.is_valid() {
                        return Err(AppError::new(1, "Erro ao adicionar Área Proibida: dimensões devem ser maiores que zero"));
                    }
                    cut_disposition_state.add_prohibited_area(&prohibited_area);
                },
            };
            Ok(())
        },
        Err(_) => Err(AppError::new(1, "Erro ao configurarar cortes")),
    }
}


#[tauri::command]
pub async fn get_piece(id: u32, state: State<'_, CutDispositionInputState>) -> Result<Rectangle, AppError> {
    let cut_disposition_state_result = state.cut_disposition_state.lock();
    match cut_disposition_state_result {
        Ok(cut_disposition_state) => {
            let item = cut_disposition_state.get_piece_by_id(id);
            match item {
                Ok(rectangle) => Ok(rectangle),
                Err(()) => Err(AppError::new(1, "Erro ao buscar Peça")),
            }
        }
        Err(_) => Err(AppError::new(1, "Erro ao buscar Peça")),
    }
}

#[tauri::command]
pub async fn get_showcase(state: State<'_, CutDispositionInputState>) -> Result<Option<Rectangle>, AppError> {
    let cut_disposition_state_result = state.cut_disposition_state.lock();
    match cut_disposition_state_result {
        Ok(cut_disposition_state) => {
            let item = cut_disposition_state.get_showcase();
            Ok(item)
        }
        Err(_) => Err(AppError::new(1, "Erro ao buscar Mostruário")),
    }
}

#[tauri::command]
pub async fn get_prohibited_area(id: u32, state: State<'_, CutDispositionInputState>) -> Result<PositionedRectangle, AppError> {
    let cut_disposition_state_result = state.cut_disposition_state.lock();
    match cut_disposition_state_result {
        Ok(cut_disposition_state) => {
            let item = cut_disposition_state.get_prohibited_area_by_id(id);
            match item {
                Ok(rectangle) => Ok(rectangle),
                Err(()) => Err(AppError::new(1, "Erro ao buscar Área Proibida")),
            }
        }
        Err(_) => Err(AppError::new(1, "Erro ao buscar Área Proibida")),
    }
}

#[tauri::command]
pub async fn edit_piece(piece: RectangleType, state: State<'_, CutDispositionInputState>) -> Result<(), AppError> {
    let cut_disposition_state_result = state.cut_disposition_state.lock();
    match cut_disposition_state_result {
        Ok(mut cut_disposition_state) => {
            match piece {
                RectangleType::Piece(main_piece) => {
                    if !main_piece.is_valid() {
                        return Err(AppError::new(1, "Erro ao adicionar Peça: dimensões devem ser maiores que zero"));
                    }
                    cut_disposition_state.edit_piece(main_piece)?;
                },
                RectangleType::Showcase(showcase) => {
                    if !showcase.is_valid() {
                        return Err(AppError::new(1, "Erro ao  adicionar Mostruário: dimensões devem ser maiores que zero"));
                    }
                    cut_disposition_state.edit_showcase(showcase)?;
                },
                RectangleType::ProhibitedArea(prohibited_area) => {
                    if !prohibited_area.is_valid() {
                        return Err(AppError::new(1, "Erro ao adicionar Área Proibida: dimensões devem ser maiores que zero"));
                    }
                    cut_disposition_state.edit_prohibited_area(prohibited_area)?;
                },
            };
            Ok(())
        },
        Err(_) => Err(AppError::new(1, "Erro ao editar peça")),
    }
}

#[tauri::command]
pub async fn delete_piece(id: u32, state: State<'_, CutDispositionInputState>) -> Result<(), AppError> {
    let cut_disposition_state_result = state.cut_disposition_state.lock();
    match cut_disposition_state_result {
        Ok(mut cut_disposition_state) => {
            let item = cut_disposition_state.remove_piece(id);
            match item {
                Ok(_rectangle) => Ok(()),
                Err(_) => Err(AppError::new(1, "Erro ao remover Peça")),
            }
        }
        Err(_) => Err(AppError::new(1, "Erro ao remover Peça")),
    }
}

#[tauri::command]
pub async fn delete_showcase(id: u32, state: State<'_, CutDispositionInputState>) -> Result<(), AppError> {
    let cut_disposition_state_result = state.cut_disposition_state.lock();
    match cut_disposition_state_result {
        Ok(mut cut_disposition_state) => {
            let item = cut_disposition_state.remove_showcase(id);
            match item {
                Ok(_rectangle) => Ok(()),
                Err(_) => Err(AppError::new(1, "Erro ao remover Peça")),
            }
        }
        Err(_) => Err(AppError::new(1, "Erro ao remover Mostruário")),
    }
}

#[tauri::command]
pub async fn delete_prohibited_area(id: u32, state: State<'_, CutDispositionInputState>) -> Result<(), AppError> {
    let cut_disposition_state_result = state.cut_disposition_state.lock();
    match cut_disposition_state_result {
        Ok(mut cut_disposition_state) => {
            let item = cut_disposition_state.remove_prohibited_area(id);
            match item {
                Ok(_rectangle) => Ok(()),
                Err(_) => Err(AppError::new(1, "Erro ao remover Área Proibida")),
            }
        }
        Err(_) => Err(AppError::new(1, "Erro ao remover Área Proibida")),
    }
}


#[tauri::command]
pub async fn organize_cut_disposition(state: State<'_, CutDispositionInputState>) -> Result<CutDispositionOutput, AppError> {
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