use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, Write},
    sync::Arc,
};

use crate::level::Level;

pub struct Options<T> {
    /// the logging level the logger should log at. default is `InfoLevel`
    level: Level,

    /// fields to always be logged
    fields: HashMap<String, T>,

    /// It's common to set this to a file, or leave it default which is `io::Stdout`
    out: Arc<RefCell<dyn Write>>,
}

impl<T> Options<T>
where
    T: Into<String> + Clone,
{
    pub fn new() -> Self {
        let out = io::stdout();
        Options {
            level: Level::InfoLevel,
            fields: HashMap::new(),
            out: Arc::new(RefCell::new(out)),
        }
    }

    // pub fn from()

    pub fn level(&self) -> Level {
        self.level.clone()
    }

    pub fn fields(&self) -> HashMap<String, T> {
        HashMap::clone(&self.fields)
    }

    pub fn out(&self) -> Arc<RefCell<dyn Write>> {
        self.out.clone()
    }

    /// set default level for the logger
    #[inline]
    pub fn with_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    /// set default fields for the logger
    #[inline]
    pub fn with_fields(mut self, fields: HashMap<String, T>) -> Self {
        self.fields = fields;
        self
    }

    /// insert key and value to the Options
    #[inline]
    pub fn insert_field(mut self, k: String, v: T) -> Self {
        self.fields.insert(k, v);
        self
    }

    /// set default output for the logger
    #[inline]
    pub fn with_out(mut self, out: Arc<RefCell<dyn Write>>) -> Self {
        self.out = out;
        self
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, collections::HashMap, io, sync::Arc};

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
            .with_out(Arc::new(RefCell::new(io::stdout())));

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
        let mut writer = rc.as_ref().borrow_mut();
        let result = writer.write(b"buf");
        assert!(result.is_ok());
    }
}
