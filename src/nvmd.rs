/// Simple syntax sugar: `.or_else(nvmd!(SomeError))`
#[macro_export]
macro_rules! nvmd {
    ($err_variant:ident($($arg:expr),+)) => {
        |$($arg),+| Err($err_variant($($arg),+))
    };
    ($error_expr:expr) => {
        |_error| Err($error_expr)
    };
}
