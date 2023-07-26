pub type ApplicationResult<T> = Result<T, ApplicationError>;
pub type SqlxResult<T> = Result<T, sqlx::Error>;

#[derive(Debug, thiserror::Error, enum_kinds::EnumKind)]
#[enum_kind(ApplicationErrorKind)]
pub enum ApplicationError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::error::Error),
    #[error(transparent)]
    LibraryError(#[from] LibraryError),
}

#[derive(Debug, thiserror::Error)]
pub enum LibraryError {
    #[error(transparent)]
    CSV(#[from] csv::Error),
    #[error("failed to collect csv deserialize result: {msg:?}")]
    CSVCollect { msg: String },
}
