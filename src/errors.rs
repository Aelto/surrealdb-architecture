pub type ApiError = Box<dyn std::error::Error>;
pub type ApiResult<T> = Result<T, ApiError>;
