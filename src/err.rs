use std::fmt;

#[derive(Debug)]
pub enum Error {
  BadRef(String),
  BadNesting(String)
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &*self {
      Error::BadRef(s) => write!(f, "Bad reference error; {}", s),
      Error::BadNesting(s) => write!(f, "Bad nesting error; {}", s)
    }
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
