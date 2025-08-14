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
        }
}