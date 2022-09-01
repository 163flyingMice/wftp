#![allow(dead_code)]
use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

pub const GET_FTPSTREAM_ERROR_CODE: usize = 501;
pub const GET_FILE_ERROR_CODE: usize = 404;
pub const DISCONNECTED_ERROR_CODE: usize = 502;
pub const CONNECTED_SUCCESS_CODE: usize = 200;
pub const UNAUTHORIZED_CODE: usize = 401;

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
pub struct Success<E> {
    pub code: usize,
    pub msg: String,
    pub list: E,
}

impl<E: Serialize> Success<E> {
    pub fn new<T>(code: usize, msg: T, list: E) -> Self
    where
        T: Display,
        E: Serialize,
    {
        Success {
            code: code,
            msg: msg.to_string(),
            list: list,
        }
    }

    pub fn out(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Debug, Deserialize)]

pub struct Error {
    pub code: usize,
    pub msg: String,
}

impl Error {
    pub fn new<T>(code: usize, msg: T) -> Self
    where
        T: Display,
    {
        Error {
            code: code,
            msg: msg.to_string(),
        }
    }

    pub fn out(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
