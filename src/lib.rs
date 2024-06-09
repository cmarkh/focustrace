pub fn setup() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("{}\n", info);
        let bt = backtrace::Backtrace::new();
        filter_backtrace(&bt);
    }));
}

fn filter_backtrace(bt: &backtrace::Backtrace) {
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();

    for frame in bt.frames() {
        for symbol in frame.symbols() {
            if let Some(function) = symbol.name() {
                let function = function.to_string();
                if function.contains(&crate_name) {
                    let file = match symbol.filename() {
                        Some(file) => match file.to_str() {
                            Some(file) => file
                                .split(format!("{}/", crate_name).as_str())
                                .last()
                                .unwrap_or_default(),
                            None => "",
                        },
                        None => "",
                    };
                    let line = symbol.lineno().unwrap_or_default();

                    eprintln!("{}:{}: {}", file, line, function);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup() {
        setup();
        panic!("Test panic");
    }
}
