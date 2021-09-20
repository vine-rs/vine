use errors::Result;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
    Error = 0,
    Request = 1,
    Response = 2,
    Event = 3,
}

type NewCodec = dyn FnOnce() -> Codec + Sync + Send + 'static;

pub trait Codec {
    fn close() -> Result<()>;
    fn string() -> &'static str;
}

pub trait Reader<T> {
    fn read_header(m: Message, mt: MessageType) -> Result<()>;
    fn read_body(body: T) -> Result<()>;
}

pub trait Writer<T> {
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
