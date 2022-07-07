use crate::error::Error;
use clap::{Arg, Command};
use std::path::PathBuf;

pub fn main() -> Result<PathBuf, Error> {
    let args = cli().get_matches();
    let path = PathBuf::from(args.get_one::<String>("input").expect("input is required"));

    match path {
        path if !path.exists() => Err(Error::InputNotFound(path)),
        path if !path.is_dir() => Err(Error::InputNotDir(path)),
        path => Ok(path),
    }
}

fn cli() -> Command<'static> {
    clap::command!().arg(
        Arg::new("input")
            .help("Path to image directory to sort")
            .value_name("DIR")
            .index(1)
            .required(true),
    )
}
