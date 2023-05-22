use std::sync::{Mutex, Weak};
use serde_json::{self, json, Value};

use xi_rpc::{RemoteError, RpcPeer};

use tracing::info;
use client::Client;
use notification::CoreNotification;
use notification::CoreNotification::{ClientStarted, TracingConfig};
use request::CoreRequest;

mod notification;
mod request;
pub mod unit_rpc;
mod client;

#[allow(dead_code)]
pub struct CoreState {
    peer: Client,
}

impl CoreState {
    pub(crate) fn new(peer: &RpcPeer) -> Self {
        CoreState {
            peer: Client::new(peer.clone()),
        }
    }

    pub(crate) fn client_notification(&mut self, cmd: CoreNotification) {
        match cmd {
            ClientStarted { .. } => (
                self.peer.send_client_started()
            ),
            TracingConfig { .. } => {}
        }
    }

    pub(crate) fn client_request(&mut self, cmd: CoreRequest) -> Result<Value, RemoteError> {
        use request::CoreRequest::*;
        match cmd {
            Version { .. } => {
                Ok(json!(1))
            }
            Initialize { .. } => {
                Ok(json!(1))
            }
        }
    }

    pub(crate) fn finish_setup(&mut self, _self_ref: WeakStadalCore) {
        self.peer.0.send_rpc_notification("config_status", &json!({ "success": true }))
    }

    pub(crate) fn handle_idle(&mut self, token: usize) {
        match token {
            _ => {
                info!("token: {}", token);
            }
        }
    }
}

/// A weak reference to the main state. This is passed to plugin threads.
#[derive(Clone)]
pub struct WeakStadalCore(Weak<Mutex<CoreState>>);
