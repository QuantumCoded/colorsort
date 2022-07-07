use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum Error {
    #[error("directory not found {0:?}")]
    InputNotFound(PathBuf),

    #[error("input {0:?} is not a directory")]
    InputNotDir(PathBuf),

    #[error("io error")]
    IoError(#[from] std::io::Error),

    #[error("image error opening {0:?}\ncaused by: {1}")]
    ImageError(PathBuf, image::ImageError),
}