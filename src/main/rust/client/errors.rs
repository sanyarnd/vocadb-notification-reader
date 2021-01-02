use actix_web::client::ConnectError;

#[derive(derive_more::Display, Debug)]
pub enum ClientError {
    #[display(fmt = "Provided username and password are not valid")]
    BadCredentialsError,

    #[display(fmt = "An invalid response received")]
    ResponseError,

    #[display(fmt = "Unable to connect to the host")]
    ConnectionError(ConnectError),

    #[display(fmt = "Unsupported argument")]
    BadArgumentError,
}

impl std::error::Error for ClientError {}

impl From<actix_web::client::SendRequestError> for ClientError {
    fn from(err: actix_web::client::SendRequestError) -> ClientError {
        return match err {
            actix_web::client::SendRequestError::Connect(e) => ClientError::ConnectionError(e),
            actix_web::client::SendRequestError::Timeout => ClientError::ResponseError,
            _ => ClientError::ResponseError,
        };
    }
}

impl From<actix_web::cookie::ParseError> for ClientError {
    fn from(_err: actix_web::cookie::ParseError) -> ClientError {
        ClientError::ResponseError
    }
}
impl From<actix_web::client::PayloadError> for ClientError {
    fn from(_err: actix_web::client::PayloadError) -> ClientError {
        ClientError::ResponseError
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(_err: serde_json::Error) -> ClientError {
        ClientError::ResponseError
    }
}

impl From<serde_urlencoded::ser::Error> for ClientError {
    fn from(_err: serde_urlencoded::ser::Error) -> ClientError {
        ClientError::BadArgumentError
    }
}
