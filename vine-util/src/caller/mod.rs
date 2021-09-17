use backtrace::Backtrace;

pub fn caller(skip: usize) -> String {
    let bt = Backtrace::new();
    let mut out = String::new();
    let frame = bt.frames().get(skip);
    if frame.is_none() {
        return out;
    }
    backtrace::resolve(frame.unwrap().ip(), |cb| {
        let filename = cb.filename();
        let lineno = cb.lineno();
        if filename.is_some() && lineno.is_some() {
            out = format!(
                "{}:{}",
                filename.unwrap().to_path_buf().to_str().unwrap(),
                lineno.unwrap()
            );
        }
    });

    out
}

#[cfg(test)]
mod test {
    use super::caller;

    #[test]
    fn test_backtrace() {
        assert_ne!(caller(5), "");
        assert_eq!(caller(100), "");
    }
}
