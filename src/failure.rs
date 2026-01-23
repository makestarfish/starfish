use async_graphql::ErrorExtensions;

macro_rules! failure_reasons {
  (
    $(
      $(#[$docs:meta])*
      ($status:expr, $id:ident, $phrase:expr),
    )+
  ) => {
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
    }
  }
}

failure_reasons! {
  /// 500 INTERNAL_SERVER_ERROR
  (500, INTERNAL_SERVER_ERROR, "Distortion in spacetime detected: internal server error"),

  /// 401 UNAUTHORIZED
  (401, UNAUTHORIZED, "Requires authentication"),

  /// 403 FORBIDDEN
  (403, FORBIDDEN, "Permission denied"),

  /// 404 NOT_FOUND
  (404, NOT_FOUND, "Resource not found"),
}

pub struct Failure {
  pub reason: FailureReason,
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
