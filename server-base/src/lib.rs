#[macro_use]
extern crate lazy_static;

mod config;
mod conn;
mod context;
mod id_gen;
mod utils;

pub use crate::config::*;
pub use crate::utils::*;
pub use conn::Conn;
pub use context::ConnContext;
pub use featureprobe_link_proto::tonic;
pub use id_gen::IdGen;
pub use minstant;
pub use tokio;
pub use tonic::transport::Channel;

use async_trait::async_trait;
use featureprobe_link_proto::packet::Message;
use parking_lot::RwLock;
use std::net::SocketAddr;

// pub type GrpcConnClient = ConnectionManagerClient<Channel>;
// pub type GrpcDispatchClient = MessageDispatchClient<Channel>;

lazy_static! {
    pub static ref USER_PORT_LISTEN: RwLock<bool> = RwLock::new(false);
    pub static ref USER_CONN_STOP: RwLock<bool> = RwLock::new(false);
}

// #[async_trait]
// pub trait Dispatch: Send + Sync {
//     async fn dispatch(&self, namespace: String, request: proto_dispatch::OnMessageRequest) -> bool;
// }

#[async_trait]
pub trait BuiltinService: Send + Sync {
    async fn on_message(&self, conn_id: &str, peer_addr: Option<SocketAddr>, message: Message);
}

// #[async_trait]
// pub trait CoreOperation: Send + Sync + Clone + 'static {
//     async fn bind_tag(&self, request: BindTagRequest) -> bool;

//     async fn unbind_tag(&self, request: UnbindTagRequest) -> bool;

//     async fn bind_tags_kv(&self, request: BindTagsKvRequest) -> bool;

//     async fn push_by_id(&self, request: PushByIdRequest) -> bool;

//     async fn push_by_tags(&self, request: PushByTagsRequest) -> PushByTagsResponse;

//     async fn get_connections_by_tag(&self, request: GetConnectionsByTagRequest) -> Vec<Connection>;

//     async fn get_exist_tags(&self, request: GetExistTagsRequest) -> Vec<String>;

//     async fn get_all_tags(&self, request: GetAllTagsRequest) -> Vec<String>;
// }

pub trait LifeCycle: Send + Sync {
    fn new_conn_id(&self, protocol: Protocol) -> String;

    fn on_conn_create(&self, conn: Conn);

    fn on_message_incoming(&self, conn_id: &str, protocol: &Protocol, message: Message);

    fn on_conn_destroy(&self, conn: Conn);

    fn should_timeout(&self) -> bool;
}

pub trait SendMessage: Send + Sync {
    #[allow(clippy::result_unit_err)]
    fn send(&self, msg: Message) -> Result<(), ()>;
}

pub trait RecvMessage: Send + Sync {
    type Item;

    #[allow(clippy::result_unit_err)]
    fn recv(&self, item: Self::Item) -> Result<Option<Message>, ()>;
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Protocol {
    Tcp,
    Websocket,
    Quic,
}

// log online time will use this
impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Protocol::Tcp => write!(f, "tcp"),
            Protocol::Websocket => write!(f, "ws"),
            Protocol::Quic => write!(f, "quic"),
        }
    }
}
