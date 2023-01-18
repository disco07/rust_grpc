use tonic::Request;
use calculator::calculator_client::CalculatorClient;
use calculator::CalculatorRequest;

pub mod calculator {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CalculatorClient::connect("http://[::1]:50051")
        .await?;

    let req = Request::new(CalculatorRequest{
        number_one: 10,
        number_two: 20,
    });

    let res = client.sum(req).await?;
    println!("{:?}", res);

    Ok(())
}