pub mod options;

pub mod etcd;

pub mod types;

use crate::options::{
    DeregisterOptions, GetOptions, ListOptions, Options, RegisterOptions, WatchOptions,
};
use crate::types::Service;

use async_trait::async_trait;
use errors::Result;
// use once_cell::sync::Lazy;

// static DefaultRegistry: Lazy<Box<dyn Registry>> = Lazy::new(|| {
//     Box::new()
// });

#[async_trait]
pub trait Registry {
    async fn init(&mut self, opt: Option<Options>) -> Result<()>;
    fn options(&self) -> Options;
    async fn register(&self, s: &Service, opt: Option<RegisterOptions>) -> Result<()>;
    async fn deregister(&self, s: &Service, opt: Option<DeregisterOptions>) -> Result<()>;
    async fn get_service(&self, s: String, opt: Option<GetOptions>) -> Result<Vec<Service>>;
    async fn list_service(&self, opt: Option<ListOptions>) -> Result<Vec<Service>>;
    async fn watch(&self, opt: Option<WatchOptions>) -> Result<Box<dyn Watcher + Send + Sync>>;
    fn string(&self) -> &'static str;
}

#[async_trait]
pub trait Watcher {
    async fn next(&self) -> Result<crate::types::Result>;
    async fn stop(&self);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
