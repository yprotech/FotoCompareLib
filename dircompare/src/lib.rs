use std::fs;
use std::path::Path;

pub fn compare(src: &str, dest: &str) {
    println!("Test only!!!",);
    println!();
    println!("Files in:{}", src);

    match get_all_files_in_directory(src) {
        Ok(files) => {
            for file in files {
                println!("{}", file);
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }

    println!();
    println!("Files in:{}", dest);

    match get_all_files_in_directory(src) {
        Ok(files) => {
            for file in files {
                println!("{}", file);
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
}

pub fn get_all_files_in_directory(directory: &str) -> Result<Vec<String>, std::io::Error> {
    let mut file_list = Vec::new();
    let path = Path::new(directory);

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(file_name) = path.to_str() {
                    file_list.push(file_name.to_string());
                }
            }
        }
    }

    Ok(file_list)
}
