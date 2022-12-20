
use rocksdb::{DBWithThreadMode, Error, SingleThreaded, DB};
use tauri::AppHandle;

const PATH: &'static str = "rocks";

pub fn connect(handle: AppHandle) -> Result<DBWithThreadMode<SingleThreaded>, Error> {
    let resources_path = handle
        .path_resolver()
        .resolve_resource(PATH.to_owned())
        .expect("Could not resolve rocks resource");

    DB::open_default(resources_path)
}

pub fn rocks_get(handle: AppHandle, key: &str) -> Result<String, Error> {
    let db = connect(handle)?;
    Ok(match db.get(key) {
        Ok(Some(value)) => String::from_utf8(value).unwrap(),
        Ok(None) => "".to_owned(),
        Err(e) => format!("Error occurred: {}", e),
    })
}

pub fn rocks_put(handle: AppHandle, key: &str, value: &str) -> Result<bool, Error> {
    let db = connect(handle)?;
    Ok(match db.put(key, value) {
        Ok(_) => true,
        Err(_) => false,
    })
}
