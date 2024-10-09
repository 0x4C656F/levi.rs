use thiserror::Error;
macro_rules! define_exception {
    ($name:ident, $code:expr, $text:expr ) => {
        #[derive(Error, Debug)]
        #[error("({} {})  ", $code, $text)]
        pub struct $name {
            status_text: String,
            status_code: u32,
            message: String,
        }

        impl $name {
            pub fn new(message: impl Into<String>) -> Self {
                $name {
                    status_code: $code,
                    status_text: $text,
                    message: message.into(),
                }
            }
        }
        impl $name {
            pub fn add_message(&mut self, s: String) -> &Self {
                self.message = s;
                self
            }
        }
        impl From<$name> for Exception {
            fn from(_: $name) -> Self {
                Exception::new($code, $text)
            }
        }
    };
}

#[derive(Error, Debug)]
#[error("({status_text} {status_code}) -> {message} ")]
pub struct Exception {
    pub status_code: u32,
    pub status_text: String,
    pub message: String,
}
impl Exception {
    pub fn new(status_code: u32, status_text: String) -> Self {
        Exception {
            status_code,
            status_text,
            message: String::new(),
        }
    }
}

define_exception!(UnauthorizedException, 401, "Unauthorized".to_string());
define_exception!(NotFoundException, 404, "Not found".to_string());
define_exception!(BadRequestException, 400, "Bad Request".to_string());
define_exception!(ForbiddenException, 403, "Forbidden".to_string());
define_exception!(
    InternalServerErrorException,
    500,
    "Internal Server Error".to_string()
);
define_exception!(NotImplementedException, 501, "Not Implemented".to_string());
define_exception!(
    ServiceUnavailableException,
    503,
    "Service Unavailable".to_string()
);
define_exception!(GatewayTimeoutException, 504, "Gateway Timeout".to_string());
define_exception!(ConflictException, 409, "Conflict".to_string());
define_exception!(
    UnprocessableEntityException,
    422,
    "Unprocessable Entity".to_string()
);
define_exception!(
    TooManyRequestsException,
    429,
    "Too Many Requests".to_string()
);
