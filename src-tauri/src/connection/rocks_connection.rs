#![allow(dead_code)]

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
