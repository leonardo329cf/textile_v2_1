use thiserror::Error;
use tokio::{fs::OpenOptions, io::{AsyncReadExt, AsyncWriteExt}};

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

pub async fn write_to_new_file(path: &str, content: &str) -> Result<(), FileError> {
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
