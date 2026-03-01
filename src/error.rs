// use std::fmt::{Display, Formatter};
use thiserror::Error as TError;

pub type APIResult<T> = Result<T, APIRequestError>;

#[derive(Debug, TError, Clone)]
pub enum APIRequestError {
	#[error("There is no result returned in time")]
	Timeout,
	#[error("The request failed with code: {:?}", code)]
	HttpError { code: HttpCode },
}

#[derive(Debug, Clone)]
pub enum HttpCode {
	Ok,
	BadRequest,
	Unauthorized,
	Forbidden,
	NotFound,
	NotAcceptable,
	Unknown(i32),
}

impl HttpCode {
	pub fn from_http_code(http_code: i32) -> Self {
		match http_code {
			200 => Self::Ok,
			400 => Self::BadRequest,
			401 => Self::Unauthorized,
			403 => Self::Forbidden,
			404 => Self::NotFound,
			406 => Self::NotAcceptable,
			other => Self::Unknown(other),
		}
	}

	pub fn retcode_to_http_code(retcode: i32) -> i32 {
		retcode - 1000
	}

	pub fn from_retcode(retcode: i32) -> Self {
		Self::from_http_code(Self::retcode_to_http_code(retcode))
	}
}

impl From<i32> for HttpCode {
	fn from(value: i32) -> Self {
		Self::from_http_code(value)
	}
}
