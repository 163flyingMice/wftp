#![allow(dead_code)]
use std::fmt;

use serde::{Deserialize, Serialize};

pub const GET_FTPSTREAM_ERROR_CODE: usize = 501;
pub const GET_FILE_ERROR_CODE: usize = 404;
pub const DISCONNECTED_ERROR_CODE: usize = 502;
pub const CONNECTED_SUCCESS_CODE: usize = 200;

#[derive(Serialize, Debug, Deserialize)]
pub enum CustomError {
    GetFtpstreamError,
}

impl std::error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::GetFtpstreamError => write!(f, "获取ftp操作对象失败！"),
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Success<T> {
    pub code: usize,
    pub msg: String,
    pub list: T,
}

impl<T> Success<T> {
    pub fn new(code: usize, msg: String, list: T) -> Self {
        Success {
            code: code,
            msg: msg,
            list: list,
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]

pub struct Error {
    pub code: usize,
    pub msg: String,
}

impl Error {
    pub fn new(code: usize, msg: String) -> Self {
        Error {
            code: code,
            msg: msg,
        }
    }
}
