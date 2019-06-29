use grpcio::{Error, RpcStatusCode};

pub fn parse(failure_err: failure::Error) {
    if let Ok(err) = failure_err.downcast::<Error>() {
        if let Error::RpcFailure(rpc_failure) = err {
            match rpc_failure.status {
                RpcStatusCode::Unavailable => {
                    eprintln!("Server is unavailable: {}", rpc_failure.details.unwrap())
                }
                _ => eprintln!("RPC error: {}", rpc_failure.details.unwrap()),
            }
        }
    }
}
