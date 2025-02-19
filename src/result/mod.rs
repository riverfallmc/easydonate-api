use serde::{Deserialize, Serialize};

/// Успешный запрос
///
/// ```json
/// {
///   "success": true,
///   "response": {
///     "message": "hello!"
///   }
/// }
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct EasySuccessfullResponse<T: Serialize> {
  success: bool,
  response: T,
}

/// Неудачный запрос
///
/// ```json
/// {
///   "success": false,
///   "response": "goodbye",
///   "error_code": 1
/// }
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct EasyErrorResponse {
  success: bool,
  response: String,
  error_code: i64
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum EasyResponse<T: Serialize> {
  Error(EasyErrorResponse),
  Successfull(EasySuccessfullResponse<T>),
}

impl<T: Serialize> EasyResponse<T> {
  pub fn result(self) -> anyhow::Result<T> {
    match self {
      EasyResponse::Successfull(success) => Ok(success.response),
      EasyResponse::Error(err) => Err(anyhow::anyhow!("{} (error_code {})", err.response, err.error_code)),
    }
  }
}

pub type EasyResult<T> = Result<T, anyhow::Error>;