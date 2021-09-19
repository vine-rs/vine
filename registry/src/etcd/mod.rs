use crate::types::Service;

pub(crate) mod lib;
pub(crate) mod watch;

static PREFIX: &str = r"/vine/registry";

fn encode(s: &Service) -> impl Into<String> {
    match serde_json::to_string(s) {
        Ok(s) => s,
        Err(_) => "".to_string(),
    }
}

fn decode<T: Into<String>>(data: T) -> Option<Service> {
    match serde_json::from_str(data.into().as_str()) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}

fn node_path<T: Into<String>>(s: T, id: T) -> String {
    let service = s.into().replace("/", "-");
    let node = id.into().replace("/", "-");
    PREFIX.to_string() + "/" + service.as_str() + "/" + node.as_str()
}

fn service_path<T: Into<String>>(s: T) -> String {
    PREFIX.to_string() + "/" + s.into().replace("/", "-").as_str()
}
