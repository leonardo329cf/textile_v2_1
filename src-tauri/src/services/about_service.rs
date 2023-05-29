use std::path::MAIN_SEPARATOR;

use thiserror::Error;

use super::file_service::{get_file_text, FileError};

#[derive(Error, Debug)]
pub enum AboutServiceError {
    #[error("Falha ao abrir ou ler arquivo localizado em: {path:?}")]
    FailedToReadFile { path: String },
}
pub async fn get_about_text() -> Result<String, AboutServiceError> {
    static FILE_NAME: &str = "about.txt";
    static CONFIG_FOLDER: &str = "configs";
    let mut about_file_path = CONFIG_FOLDER.to_string();
    about_file_path.push(MAIN_SEPARATOR);
    about_file_path.push_str(FILE_NAME);

    match get_file_text(&about_file_path).await {
        Ok(text) => Ok(text),
        Err(error) => match error {
            FileError::FailedToReadFile { path } => {
                Err(AboutServiceError::FailedToReadFile { path })
            }
        },
    }
}
