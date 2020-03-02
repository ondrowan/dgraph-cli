use std::error;
use std::fmt;

use dgraph::DgraphError;
use grpcio::{Error as GrpcError, RpcStatusCode};

#[derive(Debug)]
pub enum Error {
    DgraphError(DgraphError),
    ParsingFailed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DgraphError(err) => {
                if let DgraphError::GrpcError(GrpcError::RpcFailure(rpc_failure)) = err {
                    match rpc_failure.status {
                        RpcStatusCode::Unavailable => match &rpc_failure.details {
                            Some(details) => write!(f, "Server is unavailable: {}", details),
                            None => write!(f, "Server is unavailable."),
                        },
                        _ => match &rpc_failure.details {
                            Some(details) => write!(f, "RPC error: {}", details),
                            None => write!(f, "RPC error."),
                        },
                    }
                } else {
                    write!(f, "Dgraph error: {}", err)
                }
            }
            Error::ParsingFailed => write!(f, "Parsing of result failed."),
        }
    }
}

impl error::Error for Error {}

impl From<DgraphError> for Error {
    fn from(err: DgraphError) -> Self {
        Error::DgraphError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Error::ParsingFailed
    }
}
