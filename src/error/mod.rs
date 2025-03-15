use std::io;

#[derive(Debug)]
pub enum Error {
    InvalidInput(String),
    Io(io::Error),
    SerdeJson(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJson(err)
    }
}

impl From<Error> for tonic::Status {
    fn from(err: Error) -> Self {
        if let Error::InvalidInput(err) = err {
            return Self::new(tonic::Code::InvalidArgument, err);
        }
        Self::new(tonic::Code::Internal, format!("{:?}", err))
    }
}
