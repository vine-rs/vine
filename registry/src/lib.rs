pub mod options;

pub mod etcd;

pub mod types;

use crate::options::{
    DeregisterOptions, GetOptions, ListOptions, Options, RegisterOptions, WatchOptions,
};
use crate::types::Service;

use errors::Result;
use async_trait::async_trait;
// use once_cell::sync::Lazy;

// static DefaultRegistry: Lazy<Box<dyn Registry>> = Lazy::new(|| {
//     Box::new()
// });

#[async_trait]
pub trait Registry {
    fn init(&mut self, opt: Option<Options>) -> Result<()>;
    fn options(&self) -> Option<Options>;
    async fn register(&self, s: &Service, opt: Option<RegisterOptions>) -> Result<()>;
    async fn deregister(&self, s: &Service, opt: Option<DeregisterOptions>) -> Result<()>;
    async fn get_service(&self, s: String, opt: Option<GetOptions>) -> Result<Vec<Service>>;
    async fn list_service(&self, opt: Option<ListOptions>) -> Result<Vec<Service>>;
    async fn watch(&self, opt: Option<WatchOptions>) -> Result<Box<dyn Watcher + Send>>;
    fn string(&self) -> &'static str;
}

pub trait Watcher {
    fn next(&self) -> Result<crate::types::Result>;
    fn stop(&self);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
