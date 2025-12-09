use directories::UserDirs;
use std::path::PathBuf;

pub fn get_documents_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let user_dirs = UserDirs::new().ok_or("Could not retrieve user directories")?;
    let doc_dir = user_dirs.document_dir().ok_or("Document directory not found")?;
    Ok(doc_dir.to_path_buf())

}