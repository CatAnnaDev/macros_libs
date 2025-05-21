#[macro_export]
macro_rules! assert_some {
    ($e:expr) => {
        match $e {
            Some(val) => val,
            None => panic!("Expected Some(_), got None"),
        }
    };
}

#[macro_export]
macro_rules! assert_ok {
    ($e:expr) => {
        match $e {
            Ok(val) => val,
            Err(err) => panic!("Expected Ok(_), got Err({:?})", err),
        }
    };
}
