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
    loop {
        match stdin.next_line().await {
            Ok(Some(line)) => {
                if line.trim().is_empty(){
                    continue;
                }
                if line.eq_ignore_ascii_case("exit"){
                     println!("Exiting.");
                     break;
                }
                let msg = ChatMessage {
                    user_id: "RustClient".to_string(),
                    text: line,
                };
                if tx.send(msg).await.is_err(){
                    eprintln!("Failed to send message . Server Closed Connection.");
                    break;
                }
            }
            _ => break,
        }
    }
});