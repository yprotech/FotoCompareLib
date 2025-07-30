mod folder_infos;

use folder_infos::FolderInfo;
use std::fs;
use std::path::Path;

pub fn compare(src: &str, dest: &str) {
    println!("Test only!!!",);
    println!();
    println!("Files in:{src}");

    match get_all_files_in_directory_str(src) {
        Ok(files) => {
            for file in files.files {
                println!("{}", file.path.display());
            }
        }
        Err(e) => eprintln!("Error reading directory: {e}"),
    }

    println!();
    println!("Files in:{dest}");

    match get_all_files_in_directory_str(dest) {
        Ok(files) => {
            for file in files.files {
                println!("{}", file.path.display());
            }
        }
        Err(e) => eprintln!("Error reading directory: {e}"),
    }
}

fn get_all_files_in_directory_str(directory: &str) -> Result<FolderInfo, std::io::Error> {
    let path = Path::new(directory);

    get_all_files_in_directory(path)
}

pub fn get_all_files_in_directory(directory: &Path) -> Result<FolderInfo, std::io::Error> {
    let path = Path::new(directory);
    let canonical_path = path.canonicalize()?;
    let mut folder_info = FolderInfo::new(canonical_path);

    if path.is_dir() {
        for result_entry in fs::read_dir(path)? {
            let entry = result_entry?;
            let file_path = entry.path().canonicalize()?;

            if file_path.is_file() {
                folder_info.add_file(file_path);
            }
        }
    }

    Ok(folder_info)
}
