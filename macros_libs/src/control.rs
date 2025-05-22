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
                    }
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
macro_rules! match_result {
    ($res:expr, Ok($ok:ident) => $ok_block:block, Err($err:ident) => $err_block:block) => {
        match $res {
            Ok($ok) => $ok_block,
            Err($err) => $err_block,
        }
    };
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

#[macro_export]
macro_rules! loop_until {
    ($cond:expr, $body:block) => {{
        while !$cond {
            $body
        }
    }};
}

#[macro_export]
macro_rules! try_or_continue {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(_) => continue,
        }
    };
}

#[macro_export]
macro_rules! try_or_return {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => return Err(e.into()),
        }
    };
}

#[macro_export]
macro_rules! match_some {
    ($opt:expr, $var:ident => $some_block:block, $else_block:block) => {
        match $opt {
            Some($var) => $some_block,
            None => $else_block,
        }
    };
}

#[macro_export]
macro_rules! match_ok {
    ($res:expr, $var:ident => $ok_block:block, $else_block:block) => {
        match $res {
            Ok($var) => $ok_block,
            Err(_) => $else_block,
        }
    };
}

#[macro_export]
macro_rules! if_some {
    ($opt:expr, $var:ident => $block:block) => {
        if let Some($var) = $opt {
            $block
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_return {
    ($opt:expr) => {
        match $opt {
            Some(val) => val,
            None => return,
        }
    };
}
