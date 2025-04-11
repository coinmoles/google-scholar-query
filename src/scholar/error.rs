#[derive(Debug)]
pub enum Error {
    ConnectionError,
    ParseError,
    InvalidServiceError,
    RequiredFieldError,
    NotImplementedError,
    InvalidResponseError,
}
