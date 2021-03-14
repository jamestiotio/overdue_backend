use actix_web::{error, http::StatusCode, HttpRequest, HttpResponse};
use thiserror::Error;

use crate::models::ErrorResponse;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("A validation error has occurred.")]
    ValidationError,
    #[error("The specified resource cannot be found.")]
    NotFound,
    #[error("An invalid request has been detected.")]
    BadRequest,
    #[error("Attempted access to the specified resource is forbidden.")]
    Forbidden,
    #[error("A database error has occurred.")]
    DbError,
    #[error("An internal server error has occurred.")]
    Internal,
}

impl CustomError {
    pub fn name(&self) -> String {
        match self {
            Self::ValidationError => "Validation Error".to_string(),
            Self::NotFound => "Not Found".to_string(),
            Self::BadRequest => "Bad Request".to_string(),
            Self::Forbidden => "Forbidden Error".to_string(),
            Self::DbError => "Database Error".to_string(),
            Self::Internal => "Internal Server Error".to_string(),
        }
    }
}

impl error::ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::ValidationError => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };

        HttpResponse::build(status_code).json(error_response)
    }
}

pub fn map_io_error(e: std::io::Error) -> CustomError {
    match e.kind() {
        std::io::ErrorKind::InvalidInput => CustomError::BadRequest,
        std::io::ErrorKind::PermissionDenied => CustomError::Forbidden,
        _ => CustomError::Internal,
    }
}

pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    let status_code = StatusCode::BAD_REQUEST;
    let error_response = ErrorResponse {
        code: status_code.as_u16(),
        message: "A malformed JSON payload format has been detected.".to_string(),
        error: "Bad Request".to_string(),
    };
    let res = HttpResponse::build(status_code).json(error_response);

    error::InternalError::from_response(err, res).into()
}

pub fn query_error_handler(err: error::QueryPayloadError, _req: &HttpRequest) -> error::Error {
    let status_code = StatusCode::BAD_REQUEST;
    let error_response = ErrorResponse {
        code: status_code.as_u16(),
        message: "A malformed query format has been detected.".to_string(),
        error: "Bad Request".to_string(),
    };
    let res = HttpResponse::build(status_code).json(error_response);

    error::InternalError::from_response(err, res).into()
}

// Define unit tests for each error type
#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, ResponseError};

    use super::CustomError;

    #[test]
    fn test_default_message_validation_error() {
        let validation_error: CustomError = CustomError::ValidationError;

        assert_eq!(
            validation_error.status_code(),
            StatusCode::BAD_REQUEST,
            "Default status code should be shown"
        );
        assert_eq!(
            validation_error.name(),
            "Validation Error".to_string(),
            "Default name should be shown"
        );
        assert_eq!(
            validation_error.to_string(),
            "A validation error has occurred.".to_string(),
            "Default message should be shown"
        );
    }

    #[test]
    fn test_default_message_not_found() {
        let not_found: CustomError = CustomError::NotFound;

        assert_eq!(
            not_found.status_code(),
            StatusCode::NOT_FOUND,
            "Default status code should be shown"
        );
        assert_eq!(
            not_found.name(),
            "Not Found".to_string(),
            "Default name should be shown"
        );
        assert_eq!(
            not_found.to_string(),
            "The specified resource cannot be found.".to_string(),
            "Default message should be shown"
        );
    }

    #[test]
    fn test_default_message_bad_request() {
        let bad_request: CustomError = CustomError::BadRequest;

        assert_eq!(
            bad_request.status_code(),
            StatusCode::BAD_REQUEST,
            "Default status code should be shown"
        );
        assert_eq!(
            bad_request.name(),
            "Bad Request".to_string(),
            "Default name should be shown"
        );
        assert_eq!(
            bad_request.to_string(),
            "An invalid request has been detected.".to_string(),
            "Default message should be shown"
        );
    }

    #[test]
    fn test_default_message_forbidden() {
        let forbidden: CustomError = CustomError::Forbidden;

        assert_eq!(
            forbidden.status_code(),
            StatusCode::FORBIDDEN,
            "Default status code should be shown"
        );
        assert_eq!(
            forbidden.name(),
            "Forbidden Error".to_string(),
            "Default name should be shown"
        );
        assert_eq!(
            forbidden.to_string(),
            "Attempted access to the specified resource is forbidden.".to_string(),
            "Default message should be shown"
        );
    }

    #[test]
    fn test_default_message_db_error() {
        let db_error: CustomError = CustomError::DbError;

        assert_eq!(
            db_error.status_code(),
            StatusCode::INTERNAL_SERVER_ERROR,
            "Default status code should be shown"
        );
        assert_eq!(
            db_error.name(),
            "Database Error".to_string(),
            "Default name should be shown"
        );
        assert_eq!(
            db_error.to_string(),
            "A database error has occurred.".to_string(),
            "Default message should be shown"
        );
    }

    #[test]
    fn test_default_message_internal() {
        let internal: CustomError = CustomError::Internal;

        assert_eq!(
            internal.status_code(),
            StatusCode::INTERNAL_SERVER_ERROR,
            "Default status code should be shown"
        );
        assert_eq!(
            internal.name(),
            "Internal Server Error".to_string(),
            "Default name should be shown"
        );
        assert_eq!(
            internal.to_string(),
            "An internal server error has occurred.".to_string(),
            "Default message should be shown"
        );
    }
}
