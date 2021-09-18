use std::{
    collections::HashMap,
    io::{self, Write},
    sync::{Arc, Mutex},
};

use crate::level::Level;

#[derive(Clone)]
pub struct Options<T: Into<String> + Clone + Send> {
    /// the logging level the logger should log at. default is `InfoLevel`
    level: Level,

    skip: i32,

    /// fields to always be logged
    fields: Arc<Mutex<HashMap<String, T>>>,

    /// It's common to set this to a file, or leave it default which is `io::Stdout`
    out: Arc<Mutex<dyn Write + Send>>,
}

impl<T> Options<T>
where
    T: Into<String> + Clone + Send,
{
    pub fn new() -> Self {
        let out = io::stdout();
        Options {
            level: Level::InfoLevel,
            skip: 2,
            fields: Arc::new(Mutex::new(HashMap::new())),
            out: Arc::new(Mutex::new(out)),
        }
    }

    pub fn level(&self) -> Level {
        self.level.clone()
    }

    pub fn skip(&self) -> i32 {
        self.skip
    }

    pub fn fields(&self) -> HashMap<String, T> {
        let rc = self.fields.clone();
        if let Ok(ref mut out) = rc.lock() {
            return out.clone();
        };
        HashMap::new()
    }

    pub fn out(&self) -> Arc<Mutex<dyn Write>> {
        self.out.clone()
    }

    /// set default level for the logger
    #[inline]
    pub fn with_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    /// set default skip for the logger
    #[inline]
    pub fn with_skip(mut self, skip: i32) -> Self {
        self.skip = skip;
        self
    }

    /// set default fields for the logger
    #[inline]
    pub fn with_fields(mut self, fields: HashMap<String, T>) -> Self {
        self.fields = Arc::new(Mutex::new(fields));
        self
    }

    /// insert key and value to the Options
    #[inline]
    pub fn insert_field(self, k: String, v: T) -> Self {
        let rc = &self.fields.clone();
        if let Ok(ref mut m) = rc.lock() {
            m.insert(k, v);
        };
        self
    }

    /// set default output for the logger
    #[inline]
    pub fn with_out(mut self, out: Arc<Mutex<dyn Write + Send>>) -> Self {
        self.out = out;
        self
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::HashMap,
        io,
        sync::{Arc, Mutex},
    };

    use crate::level::Level;

    use super::Options;

    #[test]
    fn test_new() {
        let opt: Options<String> = Options::new();
        assert_eq!(opt.level(), Level::InfoLevel);
    }

    #[test]
    fn test_build() {
        let mut m = HashMap::new();
        m.insert("k".to_string(), "v".to_string());
        let mc = m.clone();
        let mut opt: Options<String> = Options::new()
            .with_level(Level::ErrorLevel)
            .with_out(Arc::new(Mutex::new(io::stdout())));

        opt = opt
            .with_fields(m)
            .insert_field("1".to_string(), "2".to_string());
        assert_eq!(opt.level(), Level::ErrorLevel);
        assert_ne!(opt.fields(), mc);
    }

    #[test]
    fn test_out() {
        let opt: Options<String> = Options::new();
        let rc = opt.out().clone();
        if let Ok(ref mut writer) = rc.lock() {
            let result = writer.write(b"buf\n");
            assert!(result.is_ok());
        };
    }
}
