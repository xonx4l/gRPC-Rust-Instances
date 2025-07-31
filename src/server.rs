use tonic::{transport::Server, Request, Response, Status};

use grpc_rust_instance::greeter::greeter_server::{Greeter, GreeterServer};
use grpc_rust_instance::greeter::{HelloReply, HelloRequest};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let name = request.into_inner().name;

        let reply = HelloReply {
            message: format!("Hello {}!", name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}