use xi_rpc::{Peer, RpcPeer};
use serde_json::json;

pub struct Client(pub(crate) RpcPeer);

impl Client {
    pub fn new(peer: RpcPeer) -> Self {
        Client(peer)
    }

    pub fn send_initialize(&self) {
        self.0.send_rpc_notification(
            "initialize",
            &json!({
                "name": "initialize",
            }),
        );
    }
}
