use grpc_rust_instance::greeter::greeter_client::GreeterClient;
use grpc_rust_instance::greeter::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Rust Developer".into(),
    });

    let response = client.say_hello(request).await?;

    println!("Response = {:?}", response.into_inner().message);

    Ok(())
} 