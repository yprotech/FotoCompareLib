use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileMetadata {
    // Placeholder for future metadata fields
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub metadata: FileMetadata,
}

#[derive(Debug, Clone)]
pub struct FolderInfo {
    pub path: PathBuf,
    pub subdirectories: Vec<FolderInfo>,
    pub files: Vec<FileInfo>,
}

impl FileInfo {
    pub fn new(path_buf: PathBuf) -> Self {
        FileInfo {
            path: path_buf,
            metadata: FileMetadata {}, // Initialize with empty metadata
        }
    }
}

impl FolderInfo {
    pub fn new(path_buf: PathBuf) -> Self {
        FolderInfo {
            path: path_buf,
            subdirectories: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn add_subdirectory(&mut self, subdirectory: FolderInfo) {
        self.subdirectories.push(subdirectory);
    }

    pub fn add_file(&mut self, path_buf: PathBuf) {
        self.files.push(FileInfo::new(path_buf));
    }
}
