use anyhow::{anyhow, Result};
use async_trait::async_trait;
use etcd_client::{Client, GetOptions as EGetOptions, PutOptions};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::str;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::options::{
    DeregisterOptions, GetOptions, ListOptions, Options, RegisterOptions, WatchOptions,
};
use crate::types::{Node, Service};
use crate::{Registry, Watcher};

pub struct EtcdRegistry {
    client: Client,
    options: Options,

    registers: Arc<Mutex<HashMap<String, u64>>>,
    leases: Arc<Mutex<HashMap<String, i64>>>,
}

static PREFIX: &str = r"/vine/registry/";

impl EtcdRegistry {
    pub async fn new(opt: Option<Options>) -> Result<Self> {
        let mut options = match opt {
            None => Options::new(),
            Some(opt) => opt,
        };

        if options.timeout == 0 {
            options.timeout = 5;
        }

        let client = Client::connect(&options.addr, None).await?;

        let eg = EtcdRegistry {
            client,
            options,
            registers: Arc::new(Mutex::new(HashMap::new())),
            leases: Arc::new(Mutex::new(HashMap::new())),
        };

        Ok(eg)
    }

    async fn register_node(
        &self,
        s: &Service,
        node: &Node,
        opt: Option<RegisterOptions>,
    ) -> Result<()> {
        if s.nodes.len() == 0 {
            return Err(anyhow!("require at lease one node"));
        }

        let mut client = self.client.clone();
        let key = format!("{}{}", s.name, node.id);
        let lease = self.leases.lock().await;
        let lease_id = lease.get(&key);
        if let None = lease_id {
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

                    self.leases
                        .lock()
                        .await
                        .insert(format!("{}{}", s.name, node.id), kv.lease());
                    self.registers
                        .lock()
                        .await
                        .insert(format!("{}{}", s.name, node.id), h.finish());
                }
            }
        }

        let mut lease_not_found = false;

        // renew the lease if it exists
        if let Some(i) = lease_id {
            println!("Renewing existing lease for {} {}", s.name, i);

            client.lease_keep_alive(i.clone()).await?;

            println!("Lease not found for {} {}", s.name, i);
            lease_not_found = true;
        }

        let mut h = DefaultHasher::new();
        node.hash(&mut h);
        let hash = h.finish();

        let registers = self.registers.lock().await;
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
        svc.nodes = Vec::new();
        svc.nodes.push(node.clone());

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
        println!(
            "Registering {} id {} with lease {} and ttl {}",
            svc.name, node.id, lease_id, ttl
        );

        client
            .put(format!("{}{}", svc.name, node.id), encode(&svc), Some(popt))
            .await?;

        self.registers
            .lock()
            .await
            .insert(format!("{}{}", svc.name, node.id), hash);
        if lease_id != 0 {
            self.leases
                .lock()
                .await
                .insert(format!("{}{}", svc.name, node.id), lease_id);
        }

        Ok(())
    }
}

#[async_trait]
impl Registry for EtcdRegistry {
    #[inline]
    fn options(&self) -> Option<Options> {
        Some(self.options.clone())
    }

    #[inline]
    async fn register(&self, s: &Service, opt: Option<RegisterOptions>) -> Result<()> {
        if s.nodes.len() == 0 {
            return Err(anyhow!("require at lease one node"));
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

    async fn deregister(&self, s: &Service, opt: Option<DeregisterOptions>) -> Result<()> {
        todo!()
    }

    async fn get_service(&self, s: &str, opt: Option<GetOptions>) -> Result<Vec<Service>> {
        todo!()
    }

    async fn list_service(&self, opt: Option<ListOptions>) -> Result<Vec<Service>> {
        todo!()
    }

    async fn watch(&self, opt: Option<WatchOptions>) -> Result<Box<dyn Watcher>> {
        todo!()
    }

    #[inline]
    fn string(&self) -> &'static str {
        "etcd"
    }
}

fn encode(s: &Service) -> String {
    match serde_json::to_string(s) {
        Ok(s) => s,
        Err(_) => "".to_string(),
    }
}

fn decode(data: &str) -> Option<Service> {
    match serde_json::from_str(data) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}

fn node_path(s: &String, id: &String) -> String {
    let service = s.replace("/", "-");
    let node = id.replace("/", "-");
    PREFIX.to_string() + "/" + service.as_str() + "/" + node.as_str()
}
