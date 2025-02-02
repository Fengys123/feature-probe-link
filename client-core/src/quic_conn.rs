use crate::Connection;
use async_trait::async_trait;
use client_proto::proto::{packet::Packet, Message};

pub struct QuicConnection {}

#[async_trait]
impl Connection for QuicConnection {
    async fn open(&self) -> bool {
        return false;
    }

    async fn send(&self, _p: Packet) -> bool {
        false
    }

    async fn close(&self) {}

    async fn state(&self) -> u8 {
        0
    }

    async fn is_same_conn(&self, _unique_id: &str) -> bool {
        false
    }
}
