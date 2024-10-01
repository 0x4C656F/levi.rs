use thiserror::Error;
macro_rules! define_exception {
    ($name:ident, $code:expr, $message:expr) => {
        #[derive(Error, Debug)]
        #[error("{} (code: {})", $message, $code)]
        pub struct $name;

        impl Default for $name {
            fn default() -> Self {
                $name
            }
        }

        impl From<$name> for Exception {
            fn from(_: $name) -> Self {
                Exception::new($code, $message)
            }
        }
    };
}

#[derive(Error, Debug)]
#[error("{message} (code: {code})")]
pub struct Exception {
    pub code: u16,
    pub message: String,
}
impl Exception {
    pub fn new(code: u16, message: impl Into<String>) -> Self {
        Exception {
            code,
            message: message.into(),
        }
    }
}

define_exception!(UnauthorizedException, 401, "Unauthorized");
