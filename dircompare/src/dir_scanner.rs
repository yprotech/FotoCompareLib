use crate::errors::ScanError;
use crate::folder_infos::FileInfo;
use std::path::Path;
use tokio::fs;
use tokio::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub enum DirectoryEvent {
    Entry(FileInfo),
    Finished,
    Error(String),
}

pub async fn get_all_files_in_directory(
    directory: &Path,
    tx: Sender<DirectoryEvent>,
) -> Result<(), ScanError> {
    let path = Path::new(directory);

    let mut dir = match fs::read_dir(&path).await {
        Ok(d) => d,
        Err(e) => {
            let _ = tx.send(DirectoryEvent::Error(e.to_string())).await;
            return Err(ScanError::IoError(e));
        }
    };

    while let Ok(Some(entry)) = dir.next_entry().await {
        let file_path = match entry.path().canonicalize() {
            Ok(p) => p,
            Err(e) => {
                let _ = tx.send(DirectoryEvent::Error(e.to_string())).await;
                continue;
            }
        };

        let fi = FileInfo::new(file_path);

        if tx.send(DirectoryEvent::Entry(fi)).await.is_err() {
            return Ok(());
        }
    }

    let _ = tx
        .send(DirectoryEvent::Finished)
        .await
        .map_err(|e| ScanError::SendError(e.to_string()))?;

    Ok(())
}
