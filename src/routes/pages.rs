use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{get, Result};

#[get("/")]
pub async fn index() -> Result<NamedFile> {
    let path: PathBuf = PathBuf::from("static/index.html");
    Ok(NamedFile::open(path)?)
}
