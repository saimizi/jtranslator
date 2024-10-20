#[allow(unused)]
use {
    error_stack::{Report, Result, ResultExt},
    jlogger_tracing::{
        jdebug, jerror, jinfo, jtrace, jwarn, JloggerBuilder, LevelFilter, LogTimeFormat,
    },
    std::{
        boxed::Box,
        ffi::{CStr, CString},
        fmt::Display,
        sync::atomic::{AtomicI32, Ordering},
    },
};

#[derive(Debug)]
pub enum JTranslateError {
    InvalidData,
    InvalidKey,
    IOError,
    UnExpected,
}

impl Display for JTranslateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            JTranslateError::InvalidData => "Invalid data",
            JTranslateError::InvalidKey => "Invalid API key",
            JTranslateError::IOError => "IO error",
            JTranslateError::UnExpected => "Unexpected error",
        };

        write!(f, "{msg}")
    }
}

impl std::error::Error for JTranslateError {}
