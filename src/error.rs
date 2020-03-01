use dgraph::DgraphError;
use grpcio::{Error, RpcStatusCode};

pub fn parse(err: DgraphError) {
    if let DgraphError::GrpcError(Error::RpcFailure(rpc_failure)) = err {
        match rpc_failure.status {
            RpcStatusCode::Unavailable => eprintln!(
                "Server is unavailable: {}",
                rpc_failure
                    .details
                    .expect("Should contain details of RPC failure.")
            ),
            _ => eprintln!(
                "RPC error: {}",
                rpc_failure
                    .details
                    .expect("Should contain details of RPC failure.")
            ),
        }
    }
}
