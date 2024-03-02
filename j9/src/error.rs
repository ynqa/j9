pub type Result<T = ()> = std::result::Result<T, Error>;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("jq_init failed")]
    JqInitError,

    #[error("NulError: {0}")]
    CStringNulError(#[from] std::ffi::NulError),

    #[error("jv_parse failed. input: {0}")]
    JvParseError(String),

    #[error("jq_compile failed. program: {0}")]
    JqCompileError(String),

    #[error("Utf8Error: {0}")]
    CStrUtf8Error(#[from] std::str::Utf8Error),
}
