extern crate failure;
extern crate serde_json;

pub mod filter;
pub mod sort;

#[cfg(test)]
pub mod test;

use failure::Fail;

type MyResult<T> = Result<T, failure::Error>;
type JsonValue = serde_json::Value;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Value error")]
    ValueError,
}
