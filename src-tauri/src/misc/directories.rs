use std::env;
use std::{
    fs::{read_dir, remove_file, DirBuilder},
    io,
    path::PathBuf,
};
use tauri::api::path::{resolve_path, BaseDirectory};
use tauri::Env;

lazy_static::lazy_static! {

    static ref TEMP_IMAGE_DIRECTORY: PathBuf = ["tests","temp_images"]
        .iter()
        .collect::<PathBuf>();

    pub static ref BASE_IMAGE_PATH: PathBuf = {
        match env::var("MINDMAP_TEST_MOD"){
            Ok(_) => TEMP_IMAGE_DIRECTORY.to_owned(),
            Err(_) => {
                let context = tauri::generate_context!();
                resolve_path(
                    context.config(),
                    context.package_info(),
                    &Env::default(),
                    "images",
                    Some(BaseDirectory::AppData),
                )
                .expect("Could not resolve base image directory path")
            }
        }
    };
    pub static ref BASE_LOG_PATH: PathBuf = {
        let context = tauri::generate_context!();
        resolve_path(
            context.config(),
            context.package_info(),
            &Env::default(),
            "log.log",
            Some(BaseDirectory::AppData),
        )
        .expect("Could not resolve base image directory path")
    };
}

pub fn create_directories() -> Result<(), io::Error> {
    DirBuilder::new()
        .recursive(true)
        .create(BASE_IMAGE_PATH.to_owned())
}

pub fn clean_temp_dirs() -> Result<(), io::Error> {
    let temp_files = read_dir(&TEMP_IMAGE_DIRECTORY.to_owned())?
        .map(|dir_entry| dir_entry.unwrap().path())
        .collect::<Vec<PathBuf>>();

    let temp_image_files = get_all_directory_image_files(&TEMP_IMAGE_DIRECTORY.to_owned())?;
    let all_files_are_images = temp_image_files.iter().len() == temp_files.clone().iter().len();
    if !all_files_are_images {
        panic!(
            "Non image files found in temp directory.
            This function is not supposed to be executed outside testing."
        );
    }

    // Now we're pretty sure that the all files are images. So we'll delete the contents of the temp directory.
    // Then we'll check again that the contents of the temp directory are indeed 0 afterwards.
    // It should be noted, that we don't have access outside $APP and $APPDATA with tauri
    // So it's not like we can delete the root directory or something
    // But We just want to be ENTIRELY certain, that we're not messing anything up when we're removing files and directories.

    temp_image_files
        .into_iter()
        .for_each(|file| remove_file(file).expect("Could not remove file in clean temp dirs"));

    // Checking again...

    if !get_all_directory_image_files(&TEMP_IMAGE_DIRECTORY.to_owned())?.len() == 0 {
        panic!(
            "Non image files found in temp directory in second check.
            This should never happen, and indicates that something is wrong with removing files.
            This function is not supposed to be executed outside testing."
        );
    }

    // And now we'll remove the directory.

    // Note: Causes some errors. Outcommented for now.
    // Image testing directory placed in tests file and added to gitignore.

    //remove_dir(&TEMP_IMAGE_DIRECTORY.to_owned())
    //    .expect("Could not remove temporary image directory");

    Ok(())
}

fn get_all_directory_image_files(directory: &PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    Ok(read_dir(directory)?
        .map(|dir_entry| dir_entry.unwrap().path())
        .collect::<Vec<PathBuf>>()
        .into_iter()
        .filter(|file| file.ends_with(".png"))
        .filter(|file| file.ends_with(".jpg"))
        .collect::<Vec<PathBuf>>())
}
