use errors::err;
use errors::Result;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Level {
    /// TraceLevel level. Designates finer-grained informational events than the Debug.
    TraceLevel = -2,

    /// DebugLevel level. Usually only enabled when debugging. Very verbose logging.
    DebugLevel = -1,

    /// InfoLevel is the default logging priority.
    /// General operational entries about what's going on inside the application.
    InfoLevel = 0,

    /// WarnLevel level. Non-critical entries that deserve eyes.
    WarnLevel = 1,

    /// ErrorLevel level. Logs. Used for errors that should definitely by noted.
    ErrorLevel = 2,

    // FatalLevel level. Logs and then calls [`logger.Exit(1)`]. Highest level of severity.
    FatalLevel = 3,
}

impl Level {
    /// retruns true if the given level is at or above this level.
    pub fn enabled(&self, lvl: &Level) -> bool {
        lvl >= self
    }

    /// converts a level string into a logger Level value.
    /// returns an error if the input string does not match known values.
    pub fn from(ls: impl Into<String>) -> Result<Level> {
        let s = ls.into();
        match s.as_str() {
            "trace" => Ok(Level::TraceLevel),
            "debug" => Ok(Level::DebugLevel),
            "info" => Ok(Level::InfoLevel),
            "warn" => Ok(Level::WarnLevel),
            "error" => Ok(Level::ErrorLevel),
            "fatal" => Ok(Level::FatalLevel),
            _ => Err(err!(
                "Unknown Level String: '{}', defaulting to InfoLevel",
                s.as_str(),
            )),
        }
    }
}

impl Into<String> for Level {
    fn into(self) -> String {
        self.to_string()
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            Level::TraceLevel => "trace",
            Level::DebugLevel => "debug",
            Level::InfoLevel => "info",
            Level::WarnLevel => "warn",
            Level::ErrorLevel => "error",
            Level::FatalLevel => "fatal",
        };
        std::fmt::Display::fmt(s, f)
    }
}

#[cfg(test)]
mod tests {
    use errors::Result;

    use crate::level::Level;

    #[test]
    fn test_level_enabled() {
        let l1 = Level::DebugLevel;
        let l2 = Level::InfoLevel;
        assert!(l1 < l2);
        println!("{}", l1.to_string());
    }

    #[test]
    fn test_from() -> Result<()> {
        let l1 = Level::from("debug")?;
        assert_eq!(l1, Level::DebugLevel);

        let l2 = Level::from("fatal")?;
        assert_eq!(l2, Level::FatalLevel);

        Ok(())
    }
}
