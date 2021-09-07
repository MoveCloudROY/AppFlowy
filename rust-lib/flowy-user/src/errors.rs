use bytes::Bytes;
use derive_more::Display;
use flowy_derive::{ProtoBuf, ProtoBuf_Enum};
use flowy_dispatch::prelude::{EventResponse, ResponseBuilder};

use std::{
    convert::TryInto,
    fmt::{Debug, Formatter},
};

#[derive(Debug, Default, Clone, ProtoBuf)]
pub struct UserError {
    #[pb(index = 1)]
    pub code: ErrorCode,

    #[pb(index = 2)]
    pub msg: String,
}

impl UserError {
    pub(crate) fn new(code: ErrorCode, msg: &str) -> Self { Self { code, msg: msg.to_owned() } }
}

#[derive(Clone, ProtoBuf_Enum, Display, PartialEq, Eq)]
pub enum ErrorCode {
    #[display(fmt = "Unknown")]
    Unknown              = 0,
    #[display(fmt = "Database init failed")]
    UserDatabaseInitFailed = 1,
    #[display(fmt = "Acquire database write lock failed")]
    AcquireWriteLockedFailed = 2,
    #[display(fmt = "Acquire database read lock failed")]
    AcquireReadLockedFailed = 3,
    #[display(fmt = "Opening database is not belonging to the current user")]
    UserDatabaseDidNotMatch = 4,
    #[display(fmt = "Database internal error")]
    UserDatabaseInternalError = 5,

    #[display(fmt = "Sql internal error")]
    SqlInternalError     = 6,

    #[display(fmt = "r2d2 connection error")]
    DatabaseConnectError = 7,

    #[display(fmt = "User not login yet")]
    UserNotLoginYet      = 10,
    #[display(fmt = "Get current id read lock failed")]
    ReadCurrentIdFailed  = 11,
    #[display(fmt = "Get current id write lock failed")]
    WriteCurrentIdFailed = 12,

    #[display(fmt = "Email can not be empty or whitespace")]
    EmailIsEmpty         = 20,
    #[display(fmt = "Email format is not valid")]
    EmailFormatInvalid   = 21,
    #[display(fmt = "Email already exists")]
    EmailAlreadyExists   = 22,
    #[display(fmt = "Password can not be empty or whitespace")]
    PasswordIsEmpty      = 30,
    #[display(fmt = "Password format too long")]
    PasswordTooLong      = 31,
    #[display(fmt = "Password contains forbidden characters.")]
    PasswordContainsForbidCharacters = 32,
    #[display(fmt = "Password should contain a minimum of 6 characters with 1 special 1 letter and 1 numeric")]
    PasswordFormatInvalid = 33,
    #[display(fmt = "Password not match")]
    PasswordNotMatch     = 34,

    #[display(fmt = "User name is too long")]
    UserNameTooLong      = 40,
    #[display(fmt = "User name contain forbidden characters")]
    UserNameContainsForbiddenCharacters = 41,
    #[display(fmt = "User name can not be empty or whitespace")]
    UserNameIsEmpty      = 42,
    #[display(fmt = "User workspace is invalid")]
    UserWorkspaceInvalid = 50,
    #[display(fmt = "User id is invalid")]
    UserIdInvalid        = 51,
    #[display(fmt = "User token is invalid")]
    UserTokenInvalid     = 54,
    #[display(fmt = "User not exist")]
    UserNotExist         = 55,

    #[display(fmt = "Create user default workspace failed")]
    CreateDefaultWorkspaceFailed = 60,

    #[display(fmt = "User default workspace already exists")]
    DefaultWorkspaceAlreadyExist = 61,

    #[display(fmt = "Server error")]
    ServerError          = 100,
}

impl Debug for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.write_str(&format!("{}", self)) }
}

impl ErrorCode {
    pub fn to_string(&self) -> String { format!("{}", self) }
}

impl std::default::Default for ErrorCode {
    fn default() -> Self { ErrorCode::Unknown }
}

impl std::convert::From<flowy_database::Error> for UserError {
    fn from(error: flowy_database::Error) -> Self { ErrorBuilder::new(ErrorCode::UserDatabaseInternalError).error(error).build() }
}

impl std::convert::From<::r2d2::Error> for UserError {
    fn from(error: r2d2::Error) -> Self { ErrorBuilder::new(ErrorCode::DatabaseConnectError).error(error).build() }
}

// use diesel::result::{Error, DatabaseErrorKind};
// use flowy_sqlite::ErrorKind;
impl std::convert::From<flowy_sqlite::Error> for UserError {
    fn from(error: flowy_sqlite::Error) -> Self { ErrorBuilder::new(ErrorCode::SqlInternalError).error(error).build() }
}

impl std::convert::From<flowy_net::errors::ServerError> for UserError {
    fn from(error: flowy_net::errors::ServerError) -> Self {
        let code = server_error_to_user_error(error.code);
        ErrorBuilder::new(code).error(error.msg).build()
    }
}

use flowy_net::errors::ErrorCode as ServerErrorCode;
fn server_error_to_user_error(code: ServerErrorCode) -> ErrorCode {
    match code {
        ServerErrorCode::InvalidToken => ErrorCode::UserTokenInvalid,
        ServerErrorCode::Unauthorized => ErrorCode::UserTokenInvalid,
        ServerErrorCode::PayloadOverflow => ErrorCode::ServerError,
        ServerErrorCode::PayloadSerdeFail => ErrorCode::ServerError,
        ServerErrorCode::PayloadUnexpectedNone => ErrorCode::ServerError,
        ServerErrorCode::ParamsInvalid => ErrorCode::ServerError,
        ServerErrorCode::ProtobufError => ErrorCode::ServerError,
        ServerErrorCode::SerdeError => ErrorCode::ServerError,
        ServerErrorCode::EmailAlreadyExists => ErrorCode::ServerError,
        ServerErrorCode::PasswordNotMatch => ErrorCode::PasswordNotMatch,
        ServerErrorCode::ConnectRefused => ErrorCode::ServerError,
        ServerErrorCode::ConnectTimeout => ErrorCode::ServerError,
        ServerErrorCode::ConnectClose => ErrorCode::ServerError,
        ServerErrorCode::ConnectCancel => ErrorCode::ServerError,
        ServerErrorCode::SqlError => ErrorCode::ServerError,
        ServerErrorCode::RecordNotFound => ErrorCode::UserNotExist,
        ServerErrorCode::HttpError => ErrorCode::ServerError,
        ServerErrorCode::InternalError => ErrorCode::ServerError,
    }
}

impl flowy_dispatch::Error for UserError {
    fn as_response(&self) -> EventResponse {
        let bytes: Bytes = self.clone().try_into().unwrap();
        ResponseBuilder::Err().data(bytes).build()
    }
}

pub type ErrorBuilder = flowy_infra::errors::Builder<ErrorCode, UserError>;

impl flowy_infra::errors::Build<ErrorCode> for UserError {
    fn build(code: ErrorCode, msg: String) -> Self {
        let msg = if msg.is_empty() { format!("{}", code) } else { msg };
        UserError::new(code, &msg)
    }
}
