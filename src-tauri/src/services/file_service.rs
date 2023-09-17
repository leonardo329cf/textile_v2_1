use std::path;

use tauri::api::path::home_dir;
use thiserror::Error;
use tokio::{fs::{OpenOptions, self}, io::{AsyncReadExt, AsyncWriteExt}};

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Falha ao abrir localizado em: {path:?}")]
    FailedToOpenFile { path: String },
    #[error("Falha ao ler conteúdo arquivo localizado em: {path:?}")]
    FailedToReadFile { path: String },
    #[error("Falha ao escrever em arquivo localizado em: {path:?}")]
    FailedToWriteFile { path: String },
}

pub async fn get_file_text(path: &str) -> Result<String, FileError> {
    let mut output = String::new();
    OpenOptions::new()
        .read(true)
        .open(path.to_owned())
        .await.map_err(|_| FileError::FailedToOpenFile {
                path: path.to_owned(),
            })?
        .read_to_string(&mut output)
        .await.map_err(|_| FileError::FailedToReadFile {
                path: path.to_owned(),
            })?;

    Ok(output)
}

pub async fn create_gcode_folder_in_home_dir_if_missing() {
    let mut path = "gcode".to_string();
    if let Some(home_path_buf) = home_dir() {
        if let Some(home_str) = home_path_buf.to_str() {
            path = format!("{}{}{}", 
            home_str, 
            path::MAIN_SEPARATOR_STR, 
            "gcode");
        }
    }

    let _ = fs::create_dir_all(path).await;
}

pub async fn write_to_new_file(path: &str, content: &str) -> Result<(), FileError> {
    create_gcode_folder_in_home_dir_if_missing().await;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path.to_owned())
        .await.map_err(|_| FileError::FailedToOpenFile {
            path: path.to_owned(),
        })?;

        file.write(content.as_bytes()).await.map_err(|_| FileError::FailedToWriteFile {
            path: path.to_owned(),
        })?;

    Ok(())
}
