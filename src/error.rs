use grpcio::Error;

pub fn parse(failure_err: failure::Error) {
    if let Ok(err) = failure_err.downcast::<Error>() {
        if let Error::RpcFailure(f) = err {
            eprintln!("RPC error: {}", f.details.unwrap());
        }
    }
}
