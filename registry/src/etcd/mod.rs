pub mod etcd;

#[cfg(test)]
mod tests {
    use crate::{Registry, types::Service};
    use std::collections::HashMap;

    use super::etcd::EtcdRegistry;
    use anyhow::Result;

    #[tokio::test]
    async fn etcd_new() {
        let reg = EtcdRegistry::new(None).await;
    }

    #[tokio::test]
    async fn etcd_register() -> Result<()> {
        let reg = EtcdRegistry::new(None).await?;

        let s = Service {
            name: "helloworld".to_string(),
            version: "".to_string(),
            metadata: HashMap::new(),
            endpoints: Vec::new(),
            nodes: Vec::new(),
            options: None,
            apis: None,
        };

        let out = reg.register(&s, None).await?;
        // println!("{}", out.);
        // assert_eq!(out, true);

        Ok(())
    }
}
