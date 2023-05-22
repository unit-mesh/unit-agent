use serde_json::json;
use xi_rpc::RpcPeer;

pub struct Client(pub(crate) RpcPeer);

impl Client {
    pub fn new(peer: RpcPeer) -> Self {
        Client(peer)
    }

    pub fn send_client_started(&self) {
        self.0.send_rpc_notification(
            "initialize",
            &json!({
                "name": "initialize",
            }),
        );
    }
}
