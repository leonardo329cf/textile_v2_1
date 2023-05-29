use thiserror::Error;
use tokio::{fs::OpenOptions, io::AsyncReadExt};

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Falha ao abrir ou ler arquivo localizado em: {path:?}")]
    FailedToReadFile { path: String },
}

pub async fn get_file_text(path: &str) -> Result<String, FileError> {
    let mut output = String::new();
    OpenOptions::new()
        .read(true)
        .open(path.to_owned())
        .await.map_err(|_| FileError::FailedToReadFile {
                path: path.to_owned(),
            })?
        .read_to_string(&mut output)
        .await.map_err(|_| FileError::FailedToReadFile {
                path: path.to_owned(),
            })?;

    Ok(output)
}
