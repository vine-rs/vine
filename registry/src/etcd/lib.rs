use async_trait::async_trait;
use errors::{bail, err, Result};
use etcd_client::{Client, ConnectOptions, GetOptions as EGetOptions, PutOptions};
use itertools::Itertools;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::str;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::watch::EtcdWatcher;
use super::{decode, encode, node_path, service_path, PREFIX};
use crate::options::{
    DeregisterOptions, GetOptions, ListOptions, Options, RegisterOptions, WatchOptions,
};
use crate::types::{Node, Service};
use crate::{Registry, Watcher};

#[derive(Clone)]
pub struct EtcdRegistry {
    client: Client,
    options: Options,

    /// 0: registers, 1: leases
    data: Arc<Mutex<(HashMap<String, u64>, HashMap<String, i64>)>>, 
}

impl EtcdRegistry {
    pub async fn new(opt: Option<Options>) -> Result<Self> {
        let mut opts = match opt {
            None => Options::new(),
            Some(opt) => opt,
        };

        if opts.timeout == 0 {
            opts.timeout = 5;
        }

        let client = Client::connect(&opts.addrs, None).await?;

        let eg = EtcdRegistry {
            client,
            options: opts,
            data: Arc::new(Mutex::new((HashMap::new(), HashMap::new()))),
        };

        Ok(eg)
    }

    async fn configure(&mut self, opt: Option<Options>) -> Result<()> {
        let mut opts = match opt {
            None => Options::new(),
            Some(opt) => opt,
        };

        if opts.timeout == 0 {
            opts.timeout = 5;
        }

        let options = {
            let copts = ConnectOptions::new();
            copts
        };

        self.client = Client::connect(&opts.addrs, Some(options)).await?;
        self.options = opts;
        self.data = Arc::new(Mutex::new((HashMap::new(), HashMap::new())));

        Ok(())
    }

    async fn register_node(
        &self,
        s: &Service,
        node: &Node,
        opt: Option<RegisterOptions>,
    ) -> Result<()> {
        if s.nodes.len() == 0 {
            return Err(err!("require at lease one node"));
        }

        let mut client = self.client.clone();
        let key = format!("{}{}", s.name, node.id);

        let data = { self.data.lock().await.clone() };
        let mut registers = data.0;
        let mut leases = data.1;

        let mut lease_not_found = false;
        match leases.get(&key) {
            None => {
                // renew the lease if it exists
                let opt = EGetOptions::new().with_serializable();
                // look for the existing key
                let rsp = client.get(node_path(&s.name, &node.id), Some(opt)).await?;

                // get the existing lease
                for kv in rsp.kvs() {
                    if kv.lease() > 0 {
                        // decode the existing node
                        let v = str::from_utf8(kv.value())?;
                        let svc = decode(v);
                        if svc.is_none() {
                            continue;
                        }
                        let s = svc.unwrap();
                        if s.nodes.len() == 0 {
                            continue;
                        }
                        let node = s.nodes.first().unwrap();
                        let mut h = DefaultHasher::new();
                        node.hash(&mut h);

                        leases.insert(format!("{}{}", s.name, node.id), kv.lease());
                        registers.insert(format!("{}{}", s.name, node.id), h.finish());
                    }
                }
            }
            Some(lease_id) if lease_id > &0 => {
                logger::debug!("Renewing existing lease for {} {}", s.name, lease_id);

                if let Err(e) = client.lease_keep_alive(lease_id.clone()).await {
                    logger::error!("Lease not found for {} {} {}", s.name, lease_id, e);
                    lease_not_found = true;
                };
            }
            _ => {}
        }

        let mut h = DefaultHasher::new();
        node.hash(&mut h);
        let hash = h.finish();

        // let registers = self.registers.lock().await;
        let v = registers.get(&format!("{}{}", s.name, node.id));
        if let Some(id) = v {
            if id == &hash && !lease_not_found {
                println!(
                    "Service {} node {} unchanged skipping registration",
                    s.name, node.id
                );
                return Ok(());
            }
        }

        let mut svc = s.clone();
        svc.nodes = vec![node.clone()];

        let mut ttl: i64 = 15;
        if let Some(o) = opt {
            ttl = o.ttl;
        }

        let mut popt = PutOptions::new();
        let lgr = client.lease_grant(ttl, None).await;
        let mut lease_id: i64 = 0;
        if let Ok(rsp) = lgr {
            lease_id = rsp.id();
            popt = popt.with_lease(rsp.id());
        }
        logger::info!(
            "Registering {} id {} with lease {} and ttl {}",
            svc.name,
            node.id,
            lease_id,
            ttl
        );

        client
            .put(
                node_path(svc.name.to_string(), node.id.to_string()),
                encode(&svc).into(),
                Some(popt),
            )
            .await?;

        registers.insert(format!("{}{}", svc.name, node.id), hash);
        if lease_id != 0 {
            leases.insert(format!("{}{}", svc.name, node.id), lease_id);
        }

        {
            let mut data = self.data.lock().await;
            data.0 = registers;
            data.1 = leases;
        }

        Ok(())
    }
}

#[async_trait]
impl Registry for EtcdRegistry {
    async fn init(&mut self, opt: Option<Options>) -> Result<()> {
        self.configure(opt).await?;
        Ok(())
    }

    #[inline]
    async fn options(&self) -> Options {
        self.options.clone()
    }

    #[inline]
    async fn register(&self, s: &Service, opt: Option<RegisterOptions>) -> Result<()> {
        if s.nodes.len() == 0 {
            return Err(err!("require at lease one node"));
        }

        let popt = match opt {
            Some(o) => o,
            None => RegisterOptions::new(),
        };
        // registry each node individually
        for node in &s.nodes {
            self.register_node(s, node, Some(popt.clone())).await?;
        }

        Ok(())
    }

    async fn deregister(&self, s: &Service, _: Option<DeregisterOptions>) -> Result<()> {
        if s.nodes.len() == 0 {
            bail!("required at lease one node")
        }

        let mut client = self.client.clone();
        for node in &s.nodes {
            logger::info!("Deregistering {} id {}", s.name, node.id);
            {
                let key = format!("{}{}", s.name, node.id);
                let mut data = self.data.lock().await;
                data.0.remove(&key);
                data.1.remove(&key);
            }

            client
                .delete(node_path(s.name.clone(), node.id.clone()), None)
                .await?;
        }

        Ok(())
    }

    async fn get_service(&self, s: String, _opt: Option<GetOptions>) -> Result<Vec<Service>> {
        let mut client = self.client.clone();

        let opts = { EGetOptions::new().with_prefix().with_serializable() };

        let key = service_path(s) + "/";
        logger::info!("{}", key);
        let rsp = client.get(key, Some(opts)).await?;
        if rsp.kvs().len() == 0 {
            // TODO: registry error
            bail!("service not found")
        }

        let mut m = HashMap::new();
        for kv in rsp.kvs() {
            let v = kv.value_str()?;
            if let Some(sn) = decode(v) {
                let version = sn.version.clone();
                let result = m.get(&version);
                if result.is_none() {
                    m.insert(version, sn);
                } else {
                    let mut s = result.unwrap().clone();
                    s.nodes = vec![s.nodes, sn.nodes].concat();
                    m.insert(version, s);
                }
            }
        }

        let services = {
            let mut slice = vec![];
            for (_, v) in &m {
                slice.push(v.clone());
            }
            slice
        };

        Ok(services)
    }

    async fn list_service(&self, _opt: Option<ListOptions>) -> Result<Vec<Service>> {
        let mut client = self.client.clone();

        let opts = { EGetOptions::new().with_prefix().with_serializable() };

        let rsp = client.get(PREFIX, Some(opts)).await?;

        let mut services = Vec::new();
        if rsp.kvs().len() == 0 {
            return Ok(services);
        }

        let mut m = HashMap::new();
        for kv in rsp.kvs() {
            let v = kv.value_str()?;
            if let Some(sn) = decode(v) {
                let version = sn.version.clone();
                let result = m.get(&version);
                if result.is_none() {
                    m.insert(version, sn);
                } else {
                    let mut s = result.unwrap().clone();
                    s.nodes = vec![s.nodes, sn.nodes].concat();
                    m.insert(version, s);
                }
            }
        }

        for v in m.keys().sorted() {
            services.push(m[v].clone());
        }

        Ok(services)
    }

    async fn watch(&self, opt: Option<WatchOptions>) -> Result<Box<dyn Watcher + Send + Sync>> {
        let watcher = EtcdWatcher::new(self.client.clone(), opt).await?;
        Ok(Box::new(watcher))
    }

    #[inline]
    async fn string(&self) -> &'static str {
        "etcd"
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        types::{Node, Service},
        Registry,
    };

    use super::EtcdRegistry;
    use errors::Result;

    #[tokio::test]
    async fn test_new_etcd_registry() -> Result<()> {
        let e = EtcdRegistry::new(None).await?;
        assert_eq!(e.string().await.to_string(), "etcd".to_string());

        Ok(())
    }

    #[tokio::test]
    async fn test_register_service() -> Result<()> {
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
        let e = EtcdRegistry::new(None).await?;

        e.register(&s, None).await?;

        e.deregister(&s, None).await?;

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
        let e = EtcdRegistry::new(None).await?;

        e.register(&s, None).await?;

        let services = e.list_service(None).await?;
        println!("{:?}", services);
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].nodes[0].id, "1");

        e.deregister(&s, None).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_service() -> Result<()> {
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
        let e = EtcdRegistry::new(None).await?;

        e.register(&s, None).await?;

        let services = e.get_service(s.name.clone(), None).await?;
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].nodes[0].id, "1");

        let result = e.get_service("ss".to_string(), None).await;
        assert!(result.is_err());

        e.deregister(&s, None).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_watch() -> Result<()> {
        let e = EtcdRegistry::new(None).await?;

        let ee = e.clone();
        tokio::spawn(async move {
            let watcher = ee.watch(None).await.unwrap();
            while let Ok(r) = watcher.next().await {
                let rr = r.clone();
                assert!(r.service.is_some());
                assert_eq!(r.service.unwrap().name, "io.vine.helloworld");
                println!("{} {:?}", rr.action, rr.service);
            }
        });

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

        e.register(&s, None).await?;

        let services = e.get_service(s.name.clone(), None).await?;
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].nodes[0].id, "1");

        let result = e.get_service("ss".to_string(), None).await;
        assert!(result.is_err());

        e.deregister(&s, None).await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        Ok(())
    }
}
