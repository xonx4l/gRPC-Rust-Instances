use tonic::{transport::Server, Request, Response, Status, Streaming};

pub mod chatter {
    tonic::include_proto!("chatter");
}

#[derive(Debug, Default)]
pub struct MyChatter {}

#[tonic::async_trait]
impl Chatter for MyChatter {
    type ChatStreamStream = 
        tokio_stream::wrappers::ReceiverStream<Result<ChatMessage, Status>>;

        async_fn_chat_stream(
            &self,
            request: Request<Streaming<ChatMessage>>,
        )-> Result<Response<Self::ChatStreamStream>,Status> {
             let mut in_Stream = request_into_inner();
             let (tx,rx) = mpsc::channel(128);

             tokio::spawn(async move {
                 while let Some(result) = in_stream_next().await {
                    match result {
                        Ok(msg) => {
                            println!("Received message from [{}]: {}", msg_user_id, msg_text);
                        let response = ChatMessage {
                            user_id = "Server".to_string(),
                            text: format!("Acknowledged your message: '{}'", msg.text),
                        };

                        if let Err(e) = tx.send(Ok(response)).await {
                            println!("Failed to send response: {}", e);
                            break;
                          }
                        }
                        Err(e) => {
                            println!("Error receiving message: {}", e);
                            break;
                        }
                    }
                 }
                 println!("Client  stream closed.");
             });
        }
}