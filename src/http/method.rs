use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

impl FromStr for Method {

    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        match s {
            "GET" => Ok(Method::GET),
            "DELETE" => Ok(Method::DELETE),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "HEAD" => Ok(Method::HEAD),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(MethodError),
        }
    }
}

impl fmt::Display for Method {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        match self {
            Method::GET => write!(f, "GET"),
            Method::DELETE => write!(f, "DELETE"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::HEAD => write!(f, "HEAD"),
            Method::CONNECT => write!(f, "CONNECT"),
            Method::OPTIONS => write!(f, "OPTIONS"),
            Method::TRACE => write!(f, "TRACE"),
            Method::PATCH => write!(f, "PATCH"),
        }
    }
}

pub struct MethodError;