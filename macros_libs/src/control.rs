#[macro_export]
macro_rules! retry {
    ($attempts:expr, $delay_ms:expr, $block:block) => {{
        use std::thread::sleep;
        use std::time::Duration;

        let mut last_err = None;

        let result = {
            let mut result = None;
            for _ in 0..$attempts {
                match $block {
                    Ok(val) => {
                        result = Some(Ok(val));
                        break;
                    },
                    Err(err) => {
                        last_err = Some(err);
                        sleep(Duration::from_millis($delay_ms));
                    }
                }
            }
            result
        };
        result.unwrap_or_else(|| Err(last_err.expect("retry! called with 0 attempts")))
    }};
}


#[macro_export]
macro_rules! exit_if_err {
    ($e:expr) => {{
        match $e {
            Ok(val) => val,
            Err(err) => {
                $crate::log_error!("{}", err);
                std::process::exit(1);
            }
        }
    }};
}

#[macro_export]
macro_rules! unwrap_or_exit {
    ($e:expr) => {{
        match $e {
            Some(val) => val,
            None => {
                $crate::log_error!("Valeur None inattendue");
                std::process::exit(1);
            }
        }
    }};
}
