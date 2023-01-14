use tonic::transport::Server;

use crate::api::{CalculatorServer, CalculatorService};

mod api;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();
    println!("Calculator listening on: {}", addr);
    let calculator = CalculatorService {};
    let svc = CalculatorServer::new(calculator);
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}
