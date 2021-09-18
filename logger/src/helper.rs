use std::{
    collections::HashMap,
    ops::Deref,
    process::exit,
    sync::{Arc, Mutex},
};

use anyhow::Result;

use crate::{level::Level, options::Options, Logger};

/// the implemention of [`Logger`] trait
///
/// ```rust
/// let l = NewLogger::<String>(Some(Options::new()))?;
/// let mut helper = Helper::new(l);
/// helper.debug(format!("debug test").as_bytes());
/// helper.info(format!("info test").as_bytes());
/// helper.warn(format!("warn test").as_bytes());
/// helper.fatal(format!("fatal test").as_bytes());
/// ```
pub struct Helper<T: Into<String> + Clone> {
    level: Level,
    log: Box<dyn Logger<T> + Send>,
    fields: Arc<Mutex<HashMap<String, T>>>,
}

impl<T> Logger<T> for Helper<T>
where
    T: Into<String> + Clone + Send,
{
    fn init(&mut self, opt: Option<Options<T>>) -> Result<()> {
        self.log.init(opt)
    }

    fn options(&self) -> Options<T> {
        self.log.options()
    }

    fn fields(&mut self, fields: HashMap<String, T>) {
        self.log.fields(fields)
    }

    fn log(&self, level: Level, arg: &[u8]) {
        self.log.log(level, arg)
    }

    fn string(&self) -> &'static str {
        self.log.string()
    }
}

impl<T> Deref for Helper<T>
where
    T: Into<String> + Clone + Sync + ?Sized,
{
    type Target = Box<dyn Logger<T> + Send>;

    fn deref(&self) -> &Self::Target {
        &self.log
    }
}

impl<T> Helper<T>
where
    T: Into<String> + Clone + Send,
{
    #[inline]
    pub fn new(log: impl Logger<T> + Send + 'static) -> Self {
        Helper {
            level: log.options().level(),
            log: Box::new(log),
            fields: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn get_fields(&self) -> HashMap<String, T> {
        if let Ok(m) = self.fields.clone().lock() {
            return m.clone();
        };
        HashMap::new()
    }

    #[inline]
    pub fn trace(&mut self, arg: &[u8]) {
        if !self.level.enabled(&Level::TraceLevel) {
            return;
        }
        self.log.fields(self.get_fields());
        self.log(Level::TraceLevel, arg);
    }

    #[inline]
    pub fn debug(&mut self, arg: &[u8]) {
        if !self.level.enabled(&Level::DebugLevel) {
            return;
        }
        self.log.fields(self.get_fields());
        self.log(Level::DebugLevel, arg);
    }

    #[inline]
    pub fn info(&mut self, arg: &[u8]) {
        if !self.level.enabled(&Level::InfoLevel) {
            return;
        }
        self.log.fields(self.get_fields());
        self.log(Level::InfoLevel, arg);
    }

    #[inline]
    pub fn warn(&mut self, arg: &[u8]) {
        if !self.level.enabled(&Level::WarnLevel) {
            return;
        }
        self.log.fields(self.get_fields());
        self.log(Level::WarnLevel, arg);
    }

    #[inline]
    pub fn error(&mut self, arg: &[u8]) {
        if !self.level.enabled(&Level::ErrorLevel) {
            return;
        }
        self.log.fields(self.get_fields());
        self.log(Level::ErrorLevel, arg);
    }

    #[inline]
    pub fn fatal(&mut self, arg: &[u8]) {
        if !self.level.enabled(&Level::FatalLevel) {
            return;
        }
        self.log.fields(self.get_fields());
        self.log(Level::FatalLevel, arg);
        exit(1);
    }

    #[inline]
    pub fn with_error(self, e: T) -> Self {
        if let Ok(ref mut m) = self.fields.clone().lock() {
            m.insert("error".to_string(), e);
        };
        self
    }

    #[inline]
    pub fn with_fields(mut self, fields: HashMap<String, T>) -> Self {
        self.fields = Arc::new(Mutex::new(fields));
        self
    }
}

#[cfg(test)]
mod test {
    use crate::{helper::Helper, options::Options, new_logger};
    use anyhow::Result;

    #[test]
    fn test_new_helper() -> Result<()> {
        let l = new_logger::<String>(Some(Options::new()))?;
        let mut helper = Helper::new(l);
        helper.debug(format!("debug test").as_bytes());
        helper.info(format!("info test").as_bytes());
        helper.warn(format!("warn test").as_bytes());
        helper.fatal(format!("fatal test").as_bytes());
        Ok(())
    }
}
