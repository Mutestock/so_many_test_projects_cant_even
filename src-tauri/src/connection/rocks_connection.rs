use rocksdb::{DBWithThreadMode, Error, SingleThreaded, DB};
use tauri::AppHandle;

const PATH: &'static str = "rocks";

pub fn connect(handle: &AppHandle) -> Result<DBWithThreadMode<SingleThreaded>, Error> {
    let resources_path = handle
        .path_resolver()
        .resolve_resource(PATH.to_owned())
        .expect("Could not resolve rocks resource");

    DB::open_default(resources_path)
}

pub fn rocks_get(handle: &AppHandle, key: &str) -> Result<String, Error> {
    Ok(match connect(handle)?.get(key) {
        Ok(Some(value)) => String::from_utf8(value).unwrap(),
        Ok(None) => "".to_owned(),
        Err(e) => format!("Error occurred: {}", e),
    })
}

pub fn rocks_put(handle: &AppHandle, key: &str, value: &str) -> Result<bool, Error> {
    Ok(match connect(handle)?.put(key, value) {
        Ok(_) => true,
        Err(_) => false,
    })
}

pub fn rocks_key_exists(handle: &AppHandle, key: &str) -> Result<bool, Error> {
    Ok(connect(handle)?.key_may_exist(key))
}

pub fn rocks_put_on_non_duplicate(
    handle: &AppHandle,
    key: &str,
    value: &str,
) -> Result<bool, Error> {
    let db = connect(handle)?;
    Ok(match db.key_may_exist(key) {
        true => false,
        false => match db.put(key, value) {
            Ok(_) => true,
            Err(_) => false,
        },
    })
}
