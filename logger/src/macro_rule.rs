#[macro_export]
macro_rules! trace {
    () => ({
        let g = $crate::global_logger().clone();
        if let Ok(ref mut m) = g.clone().lock() {
            m.trace(b"\n");
        }
    });
    ($($arg:tt)*) => ({
        let g = $crate::global_logger().clone();
        if let Ok(ref mut m) = g.clone().lock() {
            m.trace(std::format!($($arg)*).as_bytes());
        }
    })
}

#[macro_export]
macro_rules! debug {
    () => ({
        let g = $crate::global_logger().clone();
        if let Ok(ref mut m) = g.clone().lock() {
            m.debug(b"\n");
        }
    });
    ($($arg:tt)*) => ({
        let g = $crate::global_logger().clone();
        if let Ok(ref mut m) = g.clone().lock() {
            m.debug(std::format!($($arg)*).as_bytes());
        }
    })
}

#[macro_export]
macro_rules! info {
    () => ({
        let g = $crate::global_logger().clone();
        if let Ok(ref mut m) = g.clone().lock() {
            m.info(b"\n");
        }
    });
    ($($arg:tt)*) => ({
        let g = $crate::global_logger().clone();
        if let Ok(ref mut m) = g.clone().lock() {
            m.info(std::format!($($arg)*).as_bytes());
        }
    })
}

#[macro_export]
macro_rules! warn {
    () => ({
        let g = $crate::global_logger().clone();
        if let Ok(ref mut m) = g.clone().lock() {
            m.warn(b"\n");
        }
    });
    ($($arg:tt)*) => ({
        let g = $crate::global_logger().clone();
        if let Ok(ref mut m) = g.clone().lock() {
            m.warn(std::format!($($arg)*).as_bytes());
        }
    })
}

#[macro_export]
macro_rules! error {
    () => ({
        let g = $crate::global_logger().clone();
        if let Ok(ref mut m) = g.clone().lock() {
            m.error(b"\n");
        }
    });
    ($($arg:tt)*) => ({
        let g = $crate::global_logger().clone();
        if let Ok(ref mut m) = g.clone().lock() {
            m.error(std::format!($($arg)*).as_bytes());
        }
    })
}

#[macro_export]
macro_rules! fatal {
    () => ({
        let g = $crate::global_logger().clone();
        if let Ok(ref mut m) = g.clone().lock() {
            m.fatal(b"\n");
        }
    });
    ($($arg:tt)*) => ({
        let g = $crate::global_logger().clone();
        if let Ok(ref mut m) = g.clone().lock() {
            m.fatal(std::format!($($arg)*).as_bytes());
        }
    })
}

#[cfg(test)]
mod test {
    #[test]
    fn test_macro_rule() {
        trace!();
        trace!("trace");
        debug!();
        debug!("debug");
        info!();
        info!("info");
        warn!();
        warn!("warn");
        error!();
        error!("error");
    }
}