use std::sync::Arc;

use async_trait::async_trait;
use chrono::Local;
use errors::{bail, Result};
use etcd_client::{
    Client, EventType, WatchOptions as EWatchOptions, WatchStream, Watcher as EWatcher,
};
use tokio::sync::Mutex;

use crate::{options::WatchOptions, types, Watcher};

use super::{decode, service_path, PREFIX};

#[derive(Clone)]
pub struct EtcdWatcher {
    w: Arc<Mutex<(EWatcher, WatchStream)>>,
}

#[async_trait]
impl Watcher for EtcdWatcher {
    async fn next(&self) -> Result<types::Result> {
        let rc = self.w.clone();
        let mut w = rc.lock().await;
        while let Some(rsp) = w.1.message().await? {
            if rsp.canceled() {
                bail!("could not get next, watch is canceled")
            }

            for event in rsp.events() {
                if event.kv().is_none() {
                    continue;
                }

                let mut action = "";
                let mut service = types::Service::new();

                match event.event_type() {
                    EventType::Put => {
                        if let Some(kv) = event.kv() {
                            let value = kv.value_str()?;
                            if let Some(svc) = decode(value) {
                                service = svc;
                            };
                            if kv.create_revision() == kv.mod_revision() {
                                action = "create";
                            } else {
                                action = "update";
                            }
                        } else {
                            continue;
                        }
                    }
                    EventType::Delete => {
                        action = "delete";
                        if let Some(kv) = event.prev_kv() {
                            let value = kv.value_str()?;
                            if let Some(svc) = decode(value) {
                                service = svc;
                            };
                        } else {
                            continue;
                        }
                    }
                };

                logger::debug!("watch event: {} {}", action, service.name);
                let event_result = types::Result {
                    action: action.to_string().clone(),
                    service: Some(service),
                    timestamp: Local::now().timestamp(),
                };
                return Ok(event_result);
            }
        }

        bail!("could not get next")
    }

    async fn stop(&self) {
        let rc = self.w.clone();
        let mut w = rc.lock().await;
        let _ = w.0.cancel().await;
    }
}

impl EtcdWatcher {
    pub async fn new(client: Client, opt: Option<WatchOptions>) -> Result<Self> {
        let wopts = {
            let opts = EWatchOptions::new().with_prev_key().with_prefix();
            opts
        };

        let mut watch_path = PREFIX.to_string();
        if opt.is_some() {
            let o = opt.unwrap();
            if o.service != "" {
                watch_path = service_path(o.service) + "/"
            }
        };

        let w = client.clone().watch(watch_path, Some(wopts)).await?;

        let watcher = EtcdWatcher {
            w: Arc::new(Mutex::new(w)),
        };
        Ok(watcher)
    }
}
