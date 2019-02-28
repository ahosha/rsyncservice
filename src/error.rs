
/*
 * Licensed Materials - Property of IBM
 * 5725I71-CC011829
 * (C) Copyright IBM Corp. 2018 . All Rights Reserved.
 * US Government Users Restricted Rights - Use, duplication or
 * disclosure restricted by GSA ADP Schedule Contract with IBM Corp.
 */


use globset;
use notify;
use std::{fmt, io};

//pub type Result<T> = ::std::result::Result<T, Error>;

pub enum Error {
   Canonicalization(String, io::Error),
    Glob(globset::Error),
    Io(io::Error),
    Notify(notify::Error),
}

impl From<globset::Error> for Error {
    fn from(err: globset::Error) -> Self {
        Error::Glob(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<notify::Error> for Error {
    fn from(err: notify::Error) -> Self {
        match err {
            notify::Error::Io(err) => Error::Io(err),
            other => Error::Notify(other),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} error: {}",
            match self {
                Error::Canonicalization(_, _) => "Path",
                Error::Glob(_) => "Globset",
                Error::Io(_) => "I/O",
                Error::Notify(_) => "Notify",
            },
            match self {
                Error::Canonicalization(path, err) => {
                    format!("couldn't canonicalize '{}':\n{}", path, err)
                }
                err => format!("{}", err),
            }
        )
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
