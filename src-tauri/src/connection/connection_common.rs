use std::fs::DirBuilder;
use std::{io, path::PathBuf};

use tauri::{
    api::path::{resolve_path, BaseDirectory},
    Env,
};

pub trait MindmapConnector<T, Q> {
    fn connect(&self) -> Result<T, Q>;
    fn set_database_path(db_file_path: &str) -> PathBuf {
        let context = tauri::generate_context!();
        resolve_path(
            context.config(),
            context.package_info(),
            &Env::default(),
            db_file_path,
            Some(BaseDirectory::AppData),
        )
        .expect("Could not resolve sqlite path")
    }
    fn create_dir_path(&self) -> Result<(), io::Error> {
        let mut db_dir_path_without_file = self.get_dir_path().clone();
        db_dir_path_without_file.pop();
        DirBuilder::new()
            .recursive(true)
            .create(db_dir_path_without_file)
    }
    fn get_dir_path(&self) -> &PathBuf;
}
