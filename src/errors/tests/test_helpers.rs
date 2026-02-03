#[cfg(feature = "miette")]
use crate::errors::Failure;
use std::io;
use thiserror::Error;

#[allow(unused_variables)]
#[allow(clippy::absolute_paths)]
pub(crate) fn use_colors(enabled: bool) {
    #[cfg(feature = "log")]
    colored::control::set_override(enabled);
    #[cfg(feature = "miette-fancy")]
    owo_colors::set_override(enabled);
}

pub(crate) fn io_error() -> io::Error {
    io::Error::new(io::ErrorKind::NotFound, "file not found")
}

#[cfg(feature = "miette")]
pub(crate) fn http_error() -> Failure<HttpAction> {
    let json_err = io::Error::new(
        io::ErrorKind::InvalidData,
        "expected ',' at line 3 column 12",
    );
    let parse = Failure::new(HttpAction::Parse, json_err)
        .with("url", "https://api.example.com/users")
        .with("content_type", "application/json");
    Failure::new(HttpAction::CacheUsers, parse).with_path("/var/cache/users.json")
}

#[derive(Debug, Error)]
pub(crate) enum TestAction {
    #[error("read config")]
    ReadConfig,
    #[error("write file")]
    WriteFile,
    #[error("load config")]
    LoadConfig,
    #[error("parse json")]
    ParseJson,
    #[error("connect")]
    Connect,
    #[error("authenticate")]
    Authenticate,
    #[error("upload file")]
    UploadFile,
    #[error("fetch data")]
    FetchData,
    #[error("parse config file")]
    ParseConfigFile,
    #[error("get all users")]
    GetAllUsers,
}

#[derive(Debug, Error)]
pub(crate) enum HttpAction {
    #[error("parse response")]
    Parse,
    #[error("cache users")]
    CacheUsers,
}
