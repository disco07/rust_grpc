use tonic::{transport::Server, Request, Response, Status};
use calculator::calculator_server::{Calculator, CalculatorServer};
use calculator::{CalculatorRequest, CalculatorResponse};

pub mod calculator {
    tonic::include_proto!("calculator");
}

#[derive(Debug, Default)]
pub struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn sum(&self, request: Request<CalculatorRequest>) -> Result<Response<CalculatorResponse>, Status> {
        println!("Got a request {:?}", request);

        let req = request.into_inner();
        let reply = CalculatorResponse {
            result: req.number_one + req.number_two,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let calc_service = CalculatorService::default();

    Server::builder()
        .add_service(CalculatorServer::new(calc_service))
        .serve(addr)
        .await?;

    Ok(())
}