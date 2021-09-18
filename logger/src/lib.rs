use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use bytes::BufMut;
use chrono::prelude::*;
use helper::Helper;
use itertools::Itertools;
use once_cell::sync::OnceCell;

use level::Level;
use options::Options;
use vine_util::caller::caller;

pub(crate) mod helper;
pub(crate) mod level;
pub(crate) mod macro_rule;
pub(crate) mod options;

static DEFAULT_LOGGER: OnceCell<Arc<Mutex<Helper<String>>>> = OnceCell::new();
pub fn global_logger() -> &'static Arc<Mutex<Helper<String>>> {
    DEFAULT_LOGGER.get_or_init(|| {
        let l = NewLogger::<String>(Some(Options::new())).unwrap();
        let helper = Helper::new(l);
        Arc::new(Mutex::new(helper))
    })
}

pub fn set_global_logger(val: Helper<String>) -> Result<()> {
    match DEFAULT_LOGGER.set(Arc::new(Mutex::new(val))) {
        Ok(()) => Ok(()),
        Err(_) => Err(anyhow::anyhow!("set global logger failed")),
    }
}

pub trait Logger<T>
where
    T: Into<String> + Clone + Send,
{
    /// initialises options
    fn init(&mut self, opt: Option<Options<T>>) -> Result<()>;

    /// the Logger options
    fn options(&self) -> Options<T>;

    /// set fields to always be logged
    fn fields(&mut self, fields: HashMap<String, T>);

    /// writes a log entry
    fn log(&self, level: Level, arg: &[u8]);

    /// returns the name of logger
    fn string(&self) -> &'static str;
}

#[derive(Clone)]
/// The default implemention of [`Logger`] trait
/// ```rust
/// let mut l = NewLogger::<String>(Some(Options::new()))?;
/// let mut m = HashMap::new();
/// m.insert("a".to_string(), "b".to_string());
/// l.fields(m);
/// l.log(Level::InfoLevel, format!("helloworld").as_bytes());
/// ```
struct DefaultLogger<T: Into<String> + Clone + Send> {
    opts: Options<T>,
}

impl<T> Logger<T> for DefaultLogger<T>
where
    T: Into<String> + Clone + Send,
{
    fn init(&mut self, opt: Option<Options<T>>) -> Result<()> {
        let opts = match opt {
            Some(o) => o,
            None => Options::new(),
        };
        self.opts = opts;
        Ok(())
    }

    fn options(&self) -> Options<T> {
        self.opts.clone()
    }

    fn fields(&mut self, fields: HashMap<String, T>) {
        self.opts = self.opts.clone().with_fields(fields);
    }

    fn log(&self, level: Level, arg: &[u8]) {
        if !self.opts.level().enabled(&level) {
            return;
        }

        let mut fields = HashMap::new();
        for (k, v) in self.opts.fields().clone() {
            fields.insert(k, v.into());
        }
        fields.insert("level".to_string(), level.to_string());
        if !fields.contains_key("file") {
            fields.insert("file".to_string(), caller(6 + self.opts.skip() as usize));
        }

        let mut metadata = bytes::BytesMut::new();
        for key in fields.keys().sorted() {
            metadata.put_slice(format!(" {}={}", key, fields[key]).as_bytes())
        }

        let rc = self.opts.out().clone();
        if let Ok(ref mut writer) = rc.lock() {
            let local: DateTime<Local> = Local::now();

            let _ = writer.write(local.format("%Y-%m-%d %H:%M:%S").to_string().as_bytes());
            let _ = writer.write(&metadata[..]);
            let _ = writer.write(b" ");
            let _ = writer.write(arg);

            let last = arg.last();
            if last.is_some() && last.unwrap() != &10 {
                let _ = writer.write(b"\n");
            }
        };
    }

    fn string(&self) -> &'static str {
        "default"
    }
}

pub fn NewLogger<T: Into<String> + Clone + Send>(
    opts: Option<Options<T>>,
) -> Result<impl Logger<T>> {
    let opt = match opts {
        Some(o) => o,
        None => Options::new(),
    };
    let mut logger = DefaultLogger { opts: opt };
    logger.init(None)?;

    Ok(logger)
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
        thread,
    };

    use crate::{
        global_logger, level::Level, options::Options, set_global_logger, Helper, Logger, NewLogger,
    };
    use anyhow::Result;

    #[test]
    fn do_work() {
        println!("{:?}", b"\n");
    }

    #[test]
    fn test_new_logger() -> Result<()> {
        let mut l = NewLogger::<String>(Some(Options::new()))?;
        let mut m = HashMap::new();
        m.insert("a".to_string(), "b".to_string());
        l.fields(m);
        l.log(Level::InfoLevel, format!("helloworld").as_bytes());

        Ok(())
    }

    #[test]
    fn test_sync_logger() -> Result<()> {
        let l = NewLogger::<String>(Some(Options::new()))?;
        let mut helper = Helper::new(l);
        let sync_logger = Arc::new(Mutex::new(helper));

        let l1 = sync_logger.clone();
        thread::spawn(move || {
            let a = l1.lock().unwrap();

            a.log(Level::InfoLevel, "thread info".as_bytes());
        })
        .join();

        Ok(())
    }

    #[test]
    fn test_global_logger() {
        let a = global_logger().clone();
        if let Ok(ref mut m) = a.clone().lock() {
            m.info(b"hello");
        }
    }

    #[test]
    fn test_set_global_logger() -> Result<()> {
        let l = NewLogger::<String>(Some(Options::new()))?;
        let mut helper = Helper::new(l).with_error("aa".to_string());
        set_global_logger(helper)?;

        let a = global_logger().clone();
        if let Ok(ref mut m) = a.clone().lock() {
            m.info(b"hello");
        }

        Ok(())
    }
}
