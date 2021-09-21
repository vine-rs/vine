use errors::{Result, Status};

use std::{collections::HashMap, io};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
    Error = 0,
    Request = 1,
    Response = 2,
    Event = 3,
}

type NewCodec = dyn FnOnce() -> Codec + Sync + Send + 'static;

pub trait Codec {
    /// The encodable message.
    type Read: Send + 'static;
    /// The decodable message.
    type Write: Send + 'static;

    type Reader: Reader<Item = Self::Read, Error = Status> + Send + Sync + 'static;

    type Writer: Writer<Item = Self::Write, Error = Status> + Send + Sync + 'static;

    fn close() -> Result<()>;
    fn string() -> &'static str;
}

pub trait Reader<T> {
    /// The type that is encoded.
    type Item;

    /// The type of encoding errors.
    ///
    /// The type of unrecoverable frame encoding errors.
    type Error: From<io::Error>;

    fn read_header(m: Message, mt: MessageType) -> Result<()>;
    fn read_body(body: T) -> Result<()>;
}

pub trait Writer<T> {
    /// The type that is decoded.
    type Item;

    /// The type of unrecoverable frame decoding errors.
    type Error: From<io::Error>;

    fn write(m: Message, body: T) -> Result<()>;
}

/// Message represents detailed information about
/// the communication, likely followed by the body.
/// In the case of an error, body may be empty
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub id: String,
    pub r#type: MessageType,
    pub target: String,
    pub method: String,
    pub endpoint: String,
    pub error: String,

    /// The values read from the socket
    pub header: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    #[test]
    fn it_works() {
        // let mut f = File::open("foo.txt").unwrap();
        // std::io::Write
        assert_eq!(2 + 2, 4);
    }
}
