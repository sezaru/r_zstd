#[macro_export]
macro_rules! return_from_result {
    ($e:expr, $env:expr) => {
        match $e {
            Ok(inner) => return ($crate::atoms::ok(), inner).encode($env),
            Err(err) => return ($crate::atoms::error(), err).encode($env),
        }
    };
}

#[macro_export]
macro_rules! return_ok {
    ($e:expr, $env:expr) => {
        return (($crate::atoms::ok(), $e)).encode($env);
    };
    ($env:expr) => {
        return ($crate::atoms::ok()).encode($env);
    };
}
