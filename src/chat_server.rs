use tonic::{transport::Server, Request, Response, Status, Streaming};

pub mod chatter {
    tonic::include_proto!("chatter");
}

#[derive(Debug, Default)]
pub struct MyChatter {}

