pub type Result<T> = anyhow::Result<T, Error>;

/// The error type for `etcd` client.
#[derive(Debug)]
pub struct Status {
    
}

#[derive(Debug)]
pub enum Error {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
