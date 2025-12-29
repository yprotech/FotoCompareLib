mod dir_scanner;
mod errors;
mod folder_infos;

use folder_infos::FolderInfo;
use std::path::Path;
use tokio::sync::mpsc;

pub async fn compare(src: &str, dest: &str) {
    println!("Test only!!!");
    println!();
    println!("Files in:{src}");

    recv_all_files_str(src).await;

    println!();
    println!("Files in:{dest}");

    recv_all_files_str(dest).await;
}

async fn recv_all_files_str(directory: &str) {
    let path = Path::new(directory).to_path_buf();

    recv_all_files(path).await;
}

async fn recv_all_files(path: std::path::PathBuf) {
    let canonical_path = path.canonicalize().unwrap();
    let mut folder_info = FolderInfo::new(canonical_path);

    let (tx, mut rx) = mpsc::channel(100);

    let dest_handle = tokio::spawn(async move {
        if let Err(e) = dir_scanner::get_all_files_in_directory(&path, tx).await {
            eprintln!("Error scanning dest: {}", e);
        }
    });

    while let Some(event) = rx.recv().await {
        match event {
            dir_scanner::DirectoryEvent::Entry(file_info) => {
                println!("{}", file_info.path.display());
                folder_info.files.push(file_info);
            }
            dir_scanner::DirectoryEvent::Finished => break,
            dir_scanner::DirectoryEvent::Error(e) => eprintln!("Scan error (dest): {}", e),
        }
    }

    let _ = dest_handle.await;
}
