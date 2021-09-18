use errors::Result;

use crate::Watcher;

#[derive(Debug, Clone)]
pub struct EtcdWatcher {}

impl Watcher for EtcdWatcher {
    fn next(&self) -> errors::Result<crate::types::Result> {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }
}

impl EtcdWatcher {
    pub async fn new() -> Result<Self> {
        let watcher = EtcdWatcher {};
        Ok(watcher)
    }
}
