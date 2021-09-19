pub mod options;

/// #[cfg(feature = "registry-etcd")]
pub mod etcd;

pub mod types;

use crate::options::{
    DeregisterOptions, GetOptions, ListOptions, Options, RegisterOptions, WatchOptions,
};
use crate::types::Service;
use etcd::EtcdRegistry;
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell};

use async_trait::async_trait;
use errors::Result;

async fn init_registry() -> Arc<Mutex<Box<dyn Registry + Send + 'static>>> {
    let registry = EtcdRegistry::new(None).await.unwrap();
    Arc::new(Mutex::new(Box::new(registry)))
}

static DEFAULT_REGISTRY: OnceCell<Arc<Mutex<Box<dyn Registry + Send + 'static>>>> =
    OnceCell::const_new();
pub async fn global_registry() -> &'static Arc<Mutex<Box<dyn Registry + Send + 'static>>> {
    let out = DEFAULT_REGISTRY.get_or_init(init_registry).await;
    out
}

pub fn set_global_registry(reg: impl Registry + 'static) -> Result<()> {
    match DEFAULT_REGISTRY.set(Arc::new(Mutex::new(Box::new(reg)))) {
        Ok(()) => Ok(()),
        Err(_) => Err(errors::err!("set global logger failed")),
    }
}

#[async_trait]
pub trait Registry: Send {
    async fn init(&mut self, opt: Option<Options>) -> Result<()>;
    async fn options(&self) -> Options;
    async fn register(&self, s: &Service, opt: Option<RegisterOptions>) -> Result<()>;
    async fn deregister(&self, s: &Service, opt: Option<DeregisterOptions>) -> Result<()>;
    async fn get_service(&self, s: String, opt: Option<GetOptions>) -> Result<Vec<Service>>;
    async fn list_service(&self, opt: Option<ListOptions>) -> Result<Vec<Service>>;
    async fn watch(&self, opt: Option<WatchOptions>) -> Result<Box<dyn Watcher + Send + Sync>>;
    async fn string(&self) -> &'static str;
}

#[async_trait]
pub trait Watcher {
    async fn next(&self) -> Result<crate::types::Result>;
    async fn stop(&self);
}

/// register a service node. Additionally supply options such as TTL.
pub async fn register(s: &Service, opt: Option<RegisterOptions>) -> Result<()> {
    let rc = global_registry().await.clone();
    let m = rc.lock().await;
    m.register(s, opt).await?;
    Ok(())
}

/// deregister a service node
pub async fn deregister(s: &Service, opt: Option<DeregisterOptions>) -> Result<()> {
    let rc = global_registry().await.clone();
    let m = rc.lock().await;
    m.deregister(s, opt).await?;
    Ok(())
}

/// get_service retrieve a service. A slice is returned since we separate Name/Version.
pub async fn get_service(s: String, opt: Option<GetOptions>) -> Result<Vec<Service>> {
    let rc = global_registry().await.clone();
    let m = rc.lock().await;
    let services = m.get_service(s, opt).await?;
    Ok(services)
}

/// list_services list the services. Only returns service names
pub async fn list_service(opt: Option<ListOptions>) -> Result<Vec<Service>> {
    let rc = global_registry().await.clone();
    let m = rc.lock().await;
    let services = m.list_service(opt).await?;
    Ok(services)
}

/// watch returns a watcher which allows you to track updates to the registry.
pub async fn watch(opt: Option<WatchOptions>) -> Result<Box<dyn Watcher + Send + Sync>> {
    let rc = global_registry().await.clone();
    let m = rc.lock().await;
    let watcher = m.watch(opt).await?;
    Ok(watcher)
}

/// returns the name of DEFAULT_REGISTRY
pub async fn get_name() -> &'static str {
    let rc = global_registry().await.clone();
    let m = rc.lock().await;
    m.string().await
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use errors::Result;

    use crate::{deregister, etcd::EtcdRegistry, global_registry, list_service, register, set_global_registry, types::{Node, Service}};

    #[tokio::test]
    async fn test_global_registry() {
        let rc = global_registry().await.clone();
        let m = rc.lock().await;
        assert!(m.string().await != "");
    }

    #[tokio::test]
    async fn test_set_global_registry()-> Result<()> {
        let registry = EtcdRegistry::new(None).await?;
        set_global_registry(registry)?;
        let rc = global_registry().await.clone();
        let m = rc.lock().await;
        assert_eq!(m.string().await, "etcd".to_string());

        Ok(())
    }

    #[tokio::test]
    async fn test_list_service() -> Result<()> {
        let node = Node {
            id: "1".to_string(),
            address: "192.168.1.111".to_string(),
            port: 11101,
            metadata: HashMap::new(),
        };
        let s = Service {
            name: "io.vine.helloworld".to_string(),
            version: "v1.0.0".to_string(),
            metadata: HashMap::new(),
            endpoints: vec![],
            nodes: vec![node],
            options: None,
            apis: None,
        };

        register(&s, None).await?;

        let services = list_service(None).await?;
        println!("{:?}", services);
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].nodes[0].id, "1");

        deregister(&s, None).await?;

        Ok(())
    }
}
