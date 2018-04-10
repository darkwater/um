use std::convert::{From, Into};
use std::fmt;
use std::ops::Try;
use value::Value;

#[derive(Debug)]
pub enum Response<'a> {
    Success,
    Value(&'a Value),
    Error(&'static str),
}

impl<'a> Response<'a> {
    pub fn is_err(&self) -> bool {
        if let &Response::Error(_) = self {
            true
        } else {
            false
        }
    }
}

impl<'a> fmt::Display for Response<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Response::Success    => write!(f, "success"),
            Response::Value(val) => write!(f, "value {}", val),
            Response::Error(err) => write!(f, "error :{}", err),
        }
    }
}

impl<'a> From<&'a Value> for Response<'a> {
    fn from(r: &'a Value) -> Response<'a> {
        Response::Value(r)
    }
}

impl<'a, T> From<Result<T, &'static str>> for Response<'a> where
    T: Into<Response<'a>>
{
    fn from(r: Result<T, &'static str>) -> Response<'a> {
        match r {
            Ok(v)  => v.into(),
            Err(e) => Response::Error(e),
        }
    }
}

impl<'a> Try for Response<'a> {
    type Ok = Response<'a>;
    type Error = &'static str;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Response::Error(e) => Err(e),
            r @ _              => Ok(r),
        }
    }

    fn from_error(v: Self::Error) -> Self {
        Response::Error(v)
    }

    fn from_ok(v: Self::Ok) -> Self {
        v
    }
}
