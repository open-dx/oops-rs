
//---
/// Simple syntax sugar: `.or_else(nvmd!(SomeError))`
#[macro_export]
macro_rules! nvmd {
    ($error:expr) => {
        |_: _| Err($error)
    };
}
