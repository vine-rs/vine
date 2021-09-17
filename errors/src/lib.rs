use backtrace::Backtrace;
use serde::{Deserialize, Serialize};
use std::fmt;

pub type Result<T> = anyhow::Result<T>;

/// Vine status codes used by [`Status`]
/// See: https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Code {
    Unknown = 0,

    /// RFC 7231, 6.2.1
    Continue = 100,

    /// RFC 7231, 6.3.1
    Ok = 200,

    /// RFC 7231, 6.5.1
    BadRequest = 400,

    /// RFC 7235, 3.1
    Unauthorized = 401,

    /// RFC 7231, 6.5.3
    Forbidden = 403,

    /// RFC 7231, 6.5.4
    NotFound = 404,

    /// RFC 7231, 6.5.5
    MethodNotAllowed = 405,

    /// RFC 7231, 6.5.7
    RequestTimeout = 408,

    /// RFC 7209, 6.5.8
    Conflict = 409,

    /// RFC 7232, 4.2
    PreconditionFailed = 412,

    ///  RFC 6585, 4
    TooManyRequests = 429,

    /// RFC 7231, 6.6.1
    InternalServerError = 500,

    /// RFC 7231, 6.6.2
    NotImplementedError = 501,

    /// RFC 7231, 6.6.3
    BadGateway = 502,

    /// RFC 7231, 6.6.4
    ServiceUnavailable = 503,

    /// RFC 7231, 6.6.5
    GatewayTimeout = 504,
}

impl Code {
    /// Get the `Code` that represents the integer, if known.
    ///
    /// If not known, returns `Code::Unknown` (surprise!).
    pub fn from_i32(i: i32) -> Code {
        Code::from(i)
    }

    /// Convert the string representation of a `Code` (as stored, for example, in the `vine-status`
    /// header in a response) into a `Code`. Returns `Code::Unknown` if the code string is not a
    /// valid vine status code.
    pub fn description(&self) -> &'static str {
        match self {
            Code::Continue => "Continue",
            Code::Ok => "OK",
            Code::BadRequest => "Bad Request",
            Code::Unauthorized => "Unauthorized",
            Code::Forbidden => "forbidden",
            Code::NotFound => "Not Found",
            Code::MethodNotAllowed => "Method Not Allowed",
            Code::RequestTimeout => "Request Timeout",
            Code::Conflict => "Conflict",
            Code::PreconditionFailed => "Precondition Failed",
            Code::TooManyRequests => "Too Many Requests",
            Code::InternalServerError => "Internal Server Error",
            Code::NotImplementedError => "Not Implemented",
            Code::BadGateway => "Bad Gateway",
            Code::ServiceUnavailable => "Service Unavailable",
            Code::GatewayTimeout => "Gateway Timeout",
            _ => "Unknown error",
        }
    }
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.description(), f)
    }
}

impl From<i32> for Code {
    fn from(i: i32) -> Self {
        match i {
            200 => Code::Ok,
            201 => Code::Continue,
            400 => Code::BadRequest,
            401 => Code::Unauthorized,
            403 => Code::Forbidden,
            404 => Code::NotFound,
            405 => Code::MethodNotAllowed,
            408 => Code::RequestTimeout,
            409 => Code::Conflict,
            412 => Code::PreconditionFailed,
            429 => Code::TooManyRequests,
            500 => Code::InternalServerError,
            501 => Code::NotImplementedError,
            502 => Code::BadGateway,
            503 => Code::ServiceUnavailable,
            504 => Code::GatewayTimeout,

            _ => Code::Unknown,
        }
    }
}

impl From<Code> for i32 {
    #[inline]
    fn from(code: Code) -> i32 {
        code as i32
    }
}

/// A Vine status describing the result of an RPC call.
///
/// Values can be created using the `new` function or one of the specialized
/// associated functions.
/// ```rust
/// # use errors::{Status, Code};
/// let status1 = Status::new("io.vine", "name is invalid", Code::InternalServerError);
/// let status2 = Status::internal_server_error("io.vine", "name is invalid");
///
/// assert_eq!(status1.code(), Code::InternalServerError);
/// assert_eq!(status1.code(), status2.code());
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Status {
    id: String,
    code: Code,
    detail: String,
    status: String,
    position: String,
}

impl Status {
    #[inline]
    pub fn new<T: Into<String>>(id: T, detail: T, code: Code) -> Self {
        Status {
            id: id.into(),
            code,
            detail: detail.into(),
            status: code.description().to_string(),
            position: String::new(),
        }
    }

    #[inline]
    pub fn from_str(e: impl Into<String>) -> Result<Self> {
        let s = e.into();
        if let Ok(out) = serde_json::from_str(s.as_str()) {
            return Ok(out);
        }

        Ok(Status::internal_server_error("", s.as_str()))
    }

    #[inline]
    pub fn equal(&self, another: &Self) -> bool {
        self.code == another.code
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn code(&self) -> Code {
        self.code
    }

    pub fn status(&self) -> &str {
        self.status.as_str()
    }

    pub fn detail(&self) -> &str {
        self.detail.as_str()
    }

    pub fn position(&self) -> &str {
        self.position.as_str()
    }

    #[inline]
    pub fn with_id(&mut self, id: impl Into<String>) -> &Self {
        self.id = id.into();
        self
    }

    #[inline]
    pub fn with_code(&mut self, code: Code) -> &Self {
        self.code = code;
        self
    }

    #[inline]
    pub fn with_pos(&mut self) -> &Self {
        self.position = caller(5);
        self
    }

    // unknown generates a unknown error.
    pub fn unknown<T: Into<String>>(id: T, detail: T) -> Self {
        Status::new(id, detail, Code::Unknown)
    }

    // ok generates a success status
    pub fn ok(id: impl Into<String>) -> Self {
        Status {
            id: id.into(),
            code: Code::Unknown,
            detail: String::new(),
            status: Code::Unknown.to_string(),
            position: String::new(),
        }
    }

    // continue generates a contine status
    pub fn do_continue(id: impl Into<String>) -> Self {
        Status {
            id: id.into(),
            code: Code::Ok,
            detail: String::new(),
            status: Code::Ok.to_string(),
            position: String::new(),
        }
    }

    /// bad_request generates a 400 error.
    pub fn bad_request<T: Into<String>>(id: T, detail: T) -> Self {
        Status::new(id, detail, Code::BadRequest)
    }

    /// unauthorized generats a 401 error.
    pub fn unauthorized<T: Into<String>>(id: T, detail: T) -> Self {
        Status::new(id, detail, Code::Unauthorized)
    }

    /// forbidden generates a 403 error.
    pub fn forbidden<T: Into<String>>(id: T, detail: T) -> Self {
        Status::new(id, detail, Code::Forbidden)
    }

    /// not_found generates a 404 error.
    pub fn not_found<T: Into<String>>(id: T, detail: T) -> Self {
        Status::new(id, detail, Code::NotFound)
    }

    /// method_not_allowed generates a 405 error.
    pub fn method_not_allowed<T: Into<String>>(id: T, detail: T) -> Self {
        Status::new(id, detail, Code::MethodNotAllowed)
    }

    /// timeout generates a 408 error.
    pub fn timeout<T: Into<String>>(id: T, detail: T) -> Self {
        Status::new(id, detail, Code::RequestTimeout)
    }

    /// conflict generates a 409 error.
    pub fn conflict<T: Into<String>>(id: T, detail: T) -> Self {
        Status::new(id, detail, Code::Conflict)
    }

    /// precondition_failed generates a 412 error.
    pub fn precondition_failed<T: Into<String>>(id: T, detail: T) -> Self {
        Status::new(id, detail, Code::PreconditionFailed)
    }

    // too_many_requests generates a 429 error.
    pub fn too_many_requests<T: Into<String>>(id: T, detail: T) -> Self {
        Status::new(id, detail, Code::TooManyRequests)
    }

    /// internal_server_error generates a 500 error.
    pub fn internal_server_error<T: Into<String>>(id: T, detail: T) -> Self {
        Status::new(id, detail, Code::InternalServerError)
    }

    /// not_implemented generates a 501 error.
    pub fn not_implemented<T: Into<String>>(id: T, detail: T) -> Self {
        Status::new(id, detail, Code::NotImplementedError)
    }

    /// bad_gateway generates a 502 error.
    pub fn bad_gateway<T: Into<String>>(id: T, detail: T) -> Self {
        Status::new(id, detail, Code::BadGateway)
    }

    /// service_unavailable generates a 503 error.
    pub fn service_unavailable<T: Into<String>>(id: T, detail: T) -> Self {
        Status::new(id, detail, Code::ServiceUnavailable)
    }

    /// gateway_timeout generates a 504 error.
    pub fn gateway_timeout<T: Into<String>>(id: T, detail: T) -> Self {
        Status::new(id, detail, Code::GatewayTimeout)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => write!(
                f,
                "id: {}, status: {}, detail: {}",
                self.id(),
                self.status(),
                self.detail(),
            ),
        }
    }
}

impl std::error::Error for Status {}

impl From<std::io::Error> for Status {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind;
        let code = match err.kind() {
            ErrorKind::BrokenPipe
            | ErrorKind::WouldBlock
            | ErrorKind::WriteZero
            | ErrorKind::Interrupted => Code::InternalServerError,
            ErrorKind::ConnectionRefused
            | ErrorKind::ConnectionReset
            | ErrorKind::NotConnected
            | ErrorKind::AddrInUse
            | ErrorKind::AddrNotAvailable => Code::ServiceUnavailable,
            ErrorKind::AlreadyExists | ErrorKind::ConnectionAborted => Code::Conflict,
            ErrorKind::InvalidData | ErrorKind::InvalidInput => Code::BadRequest,
            ErrorKind::NotFound => Code::NotFound,
            ErrorKind::PermissionDenied => Code::Forbidden,
            ErrorKind::TimedOut => Code::RequestTimeout,
            ErrorKind::UnexpectedEof => Code::ServiceUnavailable,
            _ => Code::Unknown,
        };
        Status::new("", err.to_string().as_str(), code)
    }
}

impl From<tonic::Status> for Status {
    fn from(s: tonic::Status) -> Self {
        let code = match s.code() {
            tonic::Code::Ok => Code::Ok,
            tonic::Code::Cancelled => Code::RequestTimeout,
            tonic::Code::Unknown => Code::Unknown,
            tonic::Code::InvalidArgument => Code::BadRequest,
            tonic::Code::DeadlineExceeded => Code::GatewayTimeout,
            tonic::Code::NotFound => Code::NotFound,
            tonic::Code::AlreadyExists => Code::Conflict,
            tonic::Code::PermissionDenied => Code::Forbidden,
            tonic::Code::ResourceExhausted => Code::TooManyRequests,
            tonic::Code::FailedPrecondition => Code::PreconditionFailed,
            tonic::Code::Aborted => Code::Conflict,
            tonic::Code::OutOfRange => Code::BadRequest,
            tonic::Code::Unimplemented => Code::NotImplementedError,
            tonic::Code::Internal => Code::InternalServerError,
            tonic::Code::Unavailable => Code::ServiceUnavailable,
            tonic::Code::DataLoss => Code::InternalServerError,
            tonic::Code::Unauthenticated => Code::Unauthorized,
        };

        Status::new("", s.message(), code)
    }
}

impl Into<tonic::Status> for Status {
    fn into(self) -> tonic::Status {
        let code = match self.code() {
            Code::Unknown => tonic::Code::Unknown,
            Code::Continue |
            Code::Ok => tonic::Code::Ok,
            Code::BadRequest => tonic::Code::InvalidArgument,
            Code::Unauthorized => tonic::Code::Unauthenticated,
            Code::Forbidden => tonic::Code::PermissionDenied,
            Code::NotFound => tonic::Code::NotFound,
            Code::MethodNotAllowed => tonic::Code::Unimplemented,
            Code::RequestTimeout => tonic::Code::Cancelled,
            Code::Conflict => tonic::Code::DataLoss,
            Code::PreconditionFailed => tonic::Code::FailedPrecondition,
            Code::TooManyRequests => tonic::Code::ResourceExhausted,
            Code::InternalServerError => tonic::Code::Internal,
            Code::NotImplementedError => tonic::Code::Unimplemented,
            Code::BadGateway => tonic::Code::Internal,
            Code::ServiceUnavailable => tonic::Code::Unavailable,
            Code::GatewayTimeout => tonic::Code::DeadlineExceeded,
        };
        tonic::Status::new(code, self.detail())
    }
}

pub fn caller(skip: usize) -> String {
    let bt = Backtrace::new();
    let mut out = String::new();
    let frame = bt.frames().get(skip);
    if frame.is_none() {
        return out;
    }
    backtrace::resolve(frame.unwrap().ip(), |cb| {
        let filename = cb.filename();
        let lineno = cb.lineno();
        if filename.is_some() && lineno.is_some() {
            out = format!(
                "{}:{}",
                filename.unwrap().to_path_buf().to_str().unwrap(),
                lineno.unwrap()
            );
        }
    });

    out
}

#[cfg(test)]
mod tests {
    use crate::{caller, Code, Status};

    #[test]
    fn test_backtrace() {
        assert_ne!(caller(5), "");
        assert_eq!(caller(100), "");
    }

    #[test]
    fn test_new() {
        let status1 = Status::new("io.vine", "name is invalid", Code::InternalServerError);
        let status2 = Status::internal_server_error("io.vine", "name is invalid");

        assert_eq!(status1.code(), Code::InternalServerError);
        assert_eq!(status1.code(), status2.code());
    }

    #[test]
    fn test_from_str() {
        let e = r#"
            {"id":"io.vine","code":500,"detail":"test","status":""}
        "#;
        if let Ok(out) = Status::from_str(e) {
            assert_eq!(out.code(), Code::InternalServerError);
            return;
        }

        assert!(false)
    }

    #[test]
    fn from_tonic_status() {
        let ts = tonic::Status::new(tonic::Code::Internal, "internal error");
        let s = Status::from(ts);
        assert_eq!(s.code(), Code::InternalServerError);
        assert_eq!(s.detail(), "internal error");
    }

    #[test]
    fn into_tonic_status() {
        let s = Status::new("", "internal", Code::InternalServerError);
        // let ts = Status::into(s);
        let ts: tonic::Status = s.into();
        assert_eq!(ts.message(), "internal");
        assert_eq!(ts.code(), tonic::Code::Internal);
    }
}
