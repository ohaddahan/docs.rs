//! Simple module to store files in database.
//!
//! cratesfyi is generating more than 5 million files, they are small and mostly html files.
//! They are using so many inodes and it is better to store them in database instead of
//! filesystem. This module is adding files into database and retrieving them.

use crate::error::Result;
use crate::storage::Storage;
use postgres::Connection;

use rustc_serialize::json::{Json, ToJson};
use std::path::{Path, PathBuf};

pub(crate) use crate::storage::Blob;

pub(crate) fn get_path(conn: &Connection, path: &str) -> Option<Blob> {
    Storage::new(conn).get(path).ok()
}

/// Store all files in a directory and return [[mimetype, filename]] as Json
///
/// If there is an S3 Client configured, store files into an S3 bucket;
/// otherwise, stores files into the 'files' table of the local database.
///
/// The mimetype is detected using `magic`.
///
/// Note that this function is used for uploading both sources
/// and files generated by rustdoc.
pub fn add_path_into_database<P: AsRef<Path>>(
    conn: &Connection,
    prefix: &str,
    path: P,
) -> Result<Json> {
    let mut backend = Storage::new(conn);
    let file_list = backend.store_all(conn, prefix, path.as_ref())?;
    file_list_to_json(file_list.into_iter().collect())
}

fn file_list_to_json(file_list: Vec<(PathBuf, String)>) -> Result<Json> {
    let mut file_list_json: Vec<Json> = Vec::new();

    for file in file_list {
        let mut v: Vec<String> = Vec::with_capacity(2);
        v.push(file.1);
        v.push(file.0.into_os_string().into_string().unwrap());
        file_list_json.push(v.to_json());
    }

    Ok(file_list_json.to_json())
}
