/// Simple syntax sugar: `.or_else(nvmd!(SomeError))`
#[macro_export]
macro_rules! welp {
    ($error_expr:expr) => {
        |_error| Err($error_expr)
    };
}
