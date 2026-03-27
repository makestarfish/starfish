use async_graphql::ErrorExtensions;
use axum::{Json, response::IntoResponse};
use serde::Serialize;

macro_rules! failure_reasons {
  (
    $(
      $(#[$docs:meta])*
      ($id:ident, $phrase:expr),
    )+
  ) => {
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub enum FailureReason {
      $(
        $(#[$docs])*
        $id,
      )+
    }

    impl FailureReason {
      pub fn default_message(&self) -> &'static str {
        match self {
          $(
            FailureReason::$id => $phrase,
          )+
        }
      }

      pub fn as_str(&self) -> &str {
        match self {
          $(
            FailureReason::$id => stringify!($id),
          )+
        }
      }

      pub fn as_status_code(&self) -> axum::http::StatusCode {
        match self {
          $(
            FailureReason::$id => axum::http::StatusCode::$id,
          )+
        }
      }
    }
  }
}

failure_reasons! {
  /// 500 INTERNAL_SERVER_ERROR
  (INTERNAL_SERVER_ERROR, "Distortion in spacetime detected: internal server error"),

  /// 400 BAD_REQUEST
  (BAD_REQUEST, "The request could not be processed"),

  /// 401 UNAUTHORIZED
  (UNAUTHORIZED, "Requires authentication"),

  /// 403 FORBIDDEN
  (FORBIDDEN, "Permission denied"),

  /// 404 NOT_FOUND
  (NOT_FOUND, "Resource not found"),

  /// 409 CONFLICT
  (CONFLICT, "The request conflicts with an existing resource"),

  // 422 UNPROCESSABLE ENTITY
  (UNPROCESSABLE_ENTITY, "The request is valid but cannot be processed in the current state"),
}

#[derive(Clone)]
pub struct Failure {
  pub reason: FailureReason,
  pub message: String,
}

#[derive(Serialize)]
pub struct FailureResponse {
  pub message: String,
}

impl Failure {
  pub fn new(reason: FailureReason) -> Self {
    Self {
      message: reason.default_message().to_string(),
      reason,
    }
  }

  pub fn with_message(reason: FailureReason, message: impl ToString) -> Self {
    Self {
      reason,
      message: message.to_string(),
    }
  }
}

impl From<Failure> for async_graphql::Error {
  fn from(value: Failure) -> Self {
    Self::new(value.message)
      .extend_with(|_, e| e.set("code", value.reason.as_str()))
  }
}

impl IntoResponse for Failure {
  fn into_response(self) -> axum::response::Response {
    (
      self.reason.as_status_code(),
      Json(FailureResponse {
        message: self.message,
      }),
    )
      .into_response()
  }
}

#[macro_export]
macro_rules! failure {
  () => {
    $crate::failure::Failure::new(
      $crate::failure::FailureReason::INTERNAL_SERVER_ERROR,
    )
  };

  ($reason:expr) => {
    $crate::failure::Failure::new($reason)
  };

  ($reason:expr, $($arg:tt)*) => {
    $crate::failure::Failure::with_message($reason, format_args!($($arg)*))
  };
}

#[macro_export]
macro_rules! bail {
  () => {
    return Err(failure!())
  };

  ($reason:expr) => {
    return Err(failure!($reason))
  };

  ($reason:expr, $($arg:tt)*) => {
    return Err(failure!($reason, $($arg)*))
  };
}
