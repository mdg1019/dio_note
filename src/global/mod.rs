use crate::files;

use std::path::PathBuf;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct Global {
    pub documents_dir: PathBuf,
    pub current_dir: PathBuf,
}

pub static GLOBAL: OnceLock<Global> = OnceLock::new();

impl Global {
    pub fn init() -> Result<&'static Self, Box<dyn std::error::Error>> {
        if let Some(global) = GLOBAL.get() {
            return Ok(global);
        }
        
        let docs = files::get_documents_directory()?;
        let current = std::env::current_dir()?;
        let global = Global { documents_dir: docs, current_dir: current };
        
        Ok(GLOBAL.get_or_init(|| global))
    }
    
    pub fn get() -> Option<&'static Self> {
        GLOBAL.get()
    }
}