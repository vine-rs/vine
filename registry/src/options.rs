#[derive(Debug, Clone)]
pub struct Options {
    pub addr: Vec<String>,
    pub timeout: i64,
    pub secure: bool,
    // pub tls_config:
}

impl Options {
    #[inline]
    pub fn new() -> Self {
        Options {
            addr: vec![String::from("127.0.0.1:2379")],
            timeout: 15,
            secure: false,
        }
    }

    #[inline]
    pub fn with_timeout(&mut self, t: i64) -> &Self {
        self.timeout = t;
        self
    }

    #[inline]
    pub fn with_secure(&mut self, b: bool) -> &Self {
        self.secure = b;
        self
    }
}

#[derive(Debug, Clone)]
pub struct RegisterOptions {
    pub ttl: i64,
}

impl RegisterOptions {
    #[inline]
    pub fn new() -> Self {
        RegisterOptions { ttl: 15 }
    }

    #[inline]
    pub fn with_ttl(&mut self, ttl: i64) -> &Self {
        self.ttl = ttl;
        self
    }
}

#[derive(Debug, Clone)]
pub struct WatchOptions {
    // Specify a service to watch
    // If blank, the watch is for all services
    pub service: String,
}

impl WatchOptions {
    #[inline]
    pub fn new() -> Self {
        WatchOptions {
            service: String::new(),
        }
    }

    #[inline]
    pub fn with_service(&mut self, s: String) -> &Self {
        self.service = s;
        self
    }
}

#[derive(Debug, Clone)]
pub struct DeregisterOptions {}

#[derive(Debug, Clone)]
pub struct GetOptions {}

#[derive(Debug, Clone)]
pub struct ListOptions {}

#[derive(Debug, Clone)]
pub struct OpenapiAPIOptions {}
