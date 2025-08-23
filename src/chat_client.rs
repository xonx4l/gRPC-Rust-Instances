pub mod chatter {
    tonic::include_proto!("chatter");
}

use chatter::{chatter_client::ChatterClient, ChatMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ChatterClient::connect("https://[::1]:50052").await?;

    let (tx, rx) = mpsc::channel(32);
    let out_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
}

tokio::spawn(async move {
    let mut stdin = io::BufReader::new(io::stdin()).lines();
    println!("Enter your messages. Type 'exit' to quit.");
})