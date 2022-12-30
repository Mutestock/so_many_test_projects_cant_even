use std::{
    fs::{read_dir, remove_file, DirBuilder, remove_dir},
    io,
    path::PathBuf,
};
use tauri::api::path::BaseDirectory;

pub fn create_directories(temp_dirs: bool) -> Result<(), io::Error> {
    match temp_dirs {
        false => DirBuilder::new().recursive(true).create(
            [BaseDirectory::AppData.to_owned().variable(), "images"]
                .iter()
                .collect::<PathBuf>(),
        ),
        true => DirBuilder::new().recursive(true).create(
            [BaseDirectory::AppData.to_owned().variable(), "image_temp"]
                .iter()
                .collect::<PathBuf>(),
        ),
    }
}

pub fn clean_temp_dirs() -> Result<(), io::Error> {
    let temp_img_path = [BaseDirectory::AppData.to_owned().variable(), "image_temp"]
        .iter()
        .collect::<PathBuf>();

    let temp_files = read_dir(&temp_img_path)?
        .map(|dir_entry| dir_entry.unwrap().path())
        .collect::<Vec<PathBuf>>();

    // Rust is unhappy with cloning it directly into the statement,
    // So here's a binding.

    let temp_image_files = get_all_directory_image_files(&temp_img_path)?;

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

    if get_all_directory_image_files(&temp_img_path)?.len() == 0 {
        panic!(
            "Non image files found in temp directory in second check.
            This should never happen, and indicates that something is wrong with removing files.
            This function is not supposed to be executed outside testing."
        );
    }

    // And now we'll remove the directory.

    remove_dir(temp_img_path)
        .expect("Could not remove temporary image directory");

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
