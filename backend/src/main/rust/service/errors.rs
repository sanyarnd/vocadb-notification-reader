use crate::client::errors::VocadbClientError;

pub type Result<T, E = NotificationError> = core::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum NotificationError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error(transparent)]
    VocadbClientError(#[from] VocadbClientError),
}
