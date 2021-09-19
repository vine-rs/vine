use serde::{Deserialize, Serialize};
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

/// Service represents a vine service
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub version: String,
    pub metadata: HashMap<String, String>,
    pub endpoints: Vec<Endpoint>,
    pub nodes: Vec<Node>,
    pub options: Option<Options>,
    pub apis: Option<OpenApi>,
}

impl Service {
    pub fn new() -> Self {
        Service {
            name: "".to_string(),
            version: "".to_string(),
            metadata: HashMap::new(),
            endpoints: Vec::new(),
            nodes: Vec::new(),
            options: None,
            apis: None,
        }
    }
}

/// Node represents the node the service is on
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub address: String,
    pub port: i64,
    pub metadata: HashMap<String, String>,
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.address.hash(state);
        self.port.hash(state);
        let mut hasher = DefaultHasher::new();
        // self.metadata.hash(state);
        for (k, v) in &self.metadata {
            hasher.write(k.as_bytes());
            hasher.write(v.as_bytes());
        }
    }
}

/// Endpoint is a endpoint provided by a service
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Endpoint {
    pub name: String,
    pub request: Option<Value>,
    pub response: Option<Value>,
    pub metadata: HashMap<String, String>,
}

/// Value is an opaque value for a request or response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Value {
    pub name: String,
    pub rtype: String,
    pub values: Vec<Value>,
}

/// Options are registry options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Options {
    pub ttl: i64,
}

/// Result is returns by the watcher
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
    /// create, update, delete
    pub action: String,
    pub service: Option<Service>,
    /// unix timestamp
    pub timestamp: i64,
}

impl Result {
    pub fn new() -> Self {
        Result {
            action: "".to_string(),
            service: None,
            timestamp: 0,
        }
    }

    pub fn set_action(&mut self, action: String)  {
        self.action = action;
    }

    pub fn set_service(&mut self, service: Service)  {
        self.service = Some(service);
    }

    pub fn set_timestamp(&mut self, timestamp: i64)  {
        self.timestamp = timestamp;
    }
}

/// Event is registry event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// Event Id
    pub id: String,
    /// type of event
    #[serde(rename = "type")]
    pub r#type: i32,
    /// unix timestamp of event
    pub timestamp: i64,
    /// service entry
    pub service: Option<Service>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenApi {
    pub openapi: String,
    pub info: Option<OpenApiInfo>,
    pub external_docs: Option<OpenApiExternalDocs>,
    pub servers: Vec<OpenApiServer>,
    pub tags: Vec<OpenApiTag>,
    pub paths: HashMap<String, OpenApiPath>,
    pub components: Option<OpenApiComponents>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenApiServer {
    pub url: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenApiInfo {
    pub title: String,
    pub description: String,
    pub terms_of_service: String,
    pub contact: Option<OpenApiContact>,
    pub license: Option<OpenApiLicense>,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenApiContact {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenApiLicense {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenApiTag {
    pub name: String,
    pub description: String,
    pub external_docs: Option<OpenApiExternalDocs>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenApiExternalDocs {
    pub description: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenApiPath {
    pub get: Option<OpenApiPathDocs>,
    pub post: Option<OpenApiPathDocs>,
    pub put: Option<OpenApiPathDocs>,
    pub patch: Option<OpenApiPathDocs>,
    pub delete: Option<OpenApiPathDocs>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenApiPathDocs {
    pub tags: Vec<String>,
    pub summary: String,
    pub description: String,
    pub operation_id: String,
    pub deprecated: bool,
    pub request_body: Option<PathRequestBody>,
    pub parameters: Vec<PathParameters>,
    pub responses: HashMap<String, PathResponse>,
    pub security: Vec<PathSecurity>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathSecurity {
    pub basic: Vec<String>,
    pub api_keys: Vec<String>,
    pub bearer: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathParameters {
    /// query, cookie, path
    #[serde(rename = "in")]
    pub r#in: String,
    pub name: String,
    pub required: bool,
    pub description: String,
    pub allow_reserved: bool,
    pub style: String,
    pub explode: bool,
    pub allow_empty_value: bool,
    pub schema: Option<Schema>,
    pub example: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathRequestBody {
    pub description: String,
    pub required: bool,
    pub content: Option<PathRequestBodyContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathRequestBodyContent {
    pub application_json: Option<ApplicationContent>,
    pub application_xml: Option<ApplicationContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationContent {
    pub schema: Option<Schema>,
}

/// PathResponse is swagger path response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathResponse {
    pub description: String,
    pub content: Option<PathRequestBodyContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenApiComponents {
    pub security_schemes: Option<SecuritySchemes>,
    pub schemas: HashMap<String, Model>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecuritySchemes {
    pub basic: Option<BasicSecurity>,
    pub api_keys: Option<ApiKeysSecurity>,
    pub bearer: Option<BearerSecurity>,
}

/// BasicSecurity is swagger Basic Authorization security (https://swagger.io/docs/specification/authentication/basic-authentication/)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicSecurity {
    /// http, apiKey, oauth, openIdConnect
    #[serde(rename = "type")]
    pub r#type: String,
    pub scheme: String,
}

/// APIKeysSecurity is swagger API keys Authorization security (https://swagger.io/docs/specification/authentication/api-keys/)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiKeysSecurity {
    #[serde(rename = "type")]
    pub r#type: String,
    /// header
    #[serde(rename = "in")]
    pub r#in: String,
    pub name: String,
}

/// BearerSecurity is swagger Bearer Authorization security (https://swagger.io/docs/specification/authentication/bearer-authentication/)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BearerSecurity {
    /// http
    #[serde(rename = "type")]
    pub r#type: String,
    pub scheme: String,
    /// JWT
    pub bearer_format: String,
}

/// Model is swagger data models (https://swagger.io/docs/specification/data-models/)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Model {
    /// string, number, integer, boolean, array, object
    #[serde(rename = "type")]
    pub r#type: String,
    pub properties: HashMap<String, Schema>,
    pub required: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "type")]
    pub r#type: String,
    pub format: String,
    pub description: String,
    pub example: String,
    pub pattern: String,
    pub nullable: bool,
    pub read_only: bool,
    pub write_only: bool,
    pub required: bool,
    #[serde(rename = "ref")]
    pub r#ref: String,
    pub default: String,
    pub min_length: i32,
    pub max_length: i32,
    pub multiple_of: i32,
    pub minimum: i32,
    pub exclusive_minimum: bool,
    pub maximum: i32,
    pub exclusive_maximum: bool,
    #[serde(rename = "enum")]
    pub r#enum: Vec<String>,
    pub items: Option<Box<Schema>>,
    pub parameters: Vec<PathParameters>,
    pub additional_properties: Option<Box<Schema>>,
}
