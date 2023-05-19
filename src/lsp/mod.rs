use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard, Weak};
use serde_json::{self, json, Value};
use serde::{Deserialize, Serialize};

use xi_rpc::{Handler, RemoteError, RpcCtx, RpcPeer};

use futures::executor;
use tracing::info;
use crate::lsp::CoreNotification::{ClientStarted, SendInitialize, TracingConfig};

pub struct Client(RpcPeer);

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


#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "method", content = "params")]
pub enum CoreNotification {
    TracingConfig {
        enabled: bool,
    },
    SendInitialize {},
    ClientStarted {
        #[serde(default)]
        config_dir: Option<PathBuf>,
        #[serde(default)]
        client_extras_dir: Option<PathBuf>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "method", content = "params")]
pub enum CoreRequest {
    GetConfig {},
}

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
            SendInitialize {} => {
                self.peer.send_initialize();
            }
            ClientStarted { .. } => (),
            _ => {
                // self.not_command(view_id, language_id);
            }
        }
    }

    pub(crate) fn client_request(&mut self, cmd: CoreRequest) -> Result<Value, RemoteError> {
        use self::CoreRequest::*;
        match cmd {
            GetConfig {} => Ok(json!(1)),
        }
    }

    pub(crate) fn finish_setup(&mut self, self_ref: WeakStadalCore) {
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

pub enum Ulsp {
    // TODO: profile startup, and determine what things (such as theme loading)
    // we should be doing before client_init.
    Waiting,
    Running(Arc<Mutex<CoreState>>),
}

/// A weak reference to the main state. This is passed to plugin threads.
#[derive(Clone)]
pub struct WeakStadalCore(Weak<Mutex<CoreState>>);

#[allow(dead_code)]
impl Ulsp {
    pub fn new() -> Self {
        Ulsp::Waiting
    }

    /// Returns `true` if the `client_started` has not been received.
    fn is_waiting(&self) -> bool {
        match *self {
            Ulsp::Waiting => true,
            _ => false,
        }
    }

    /// Returns a guard to the core state. A convenience around `Mutex::lock`.
    ///
    /// # Panics
    ///
    /// Panics if core has not yet received the `client_started` message.
    pub fn inner(&self) -> MutexGuard<CoreState> {
        match self {
            Ulsp::Running(ref inner) => inner.lock().unwrap(),
            Ulsp::Waiting => panic!(
                "core does not start until client_started \
                 RPC is received"
            ),
        }
    }

    /// Returns a new reference to the core state, if core is running.
    fn weak_self(&self) -> Option<WeakStadalCore> {
        match self {
            Ulsp::Running(ref inner) => Some(WeakStadalCore(Arc::downgrade(inner))),
            Ulsp::Waiting => None,
        }
    }
}

impl Handler for Ulsp {
    type Notification = CoreNotification;
    type Request = CoreRequest;

    fn handle_notification(&mut self, ctx: &RpcCtx, rpc: Self::Notification) {
        // We allow tracing to be enabled before event `client_started`
        if let TracingConfig { enabled } = rpc {
            info!("tracing in core = {:?}", enabled);
            if self.is_waiting() {
                return;
            }
        }

        if let ClientStarted {
            ref config_dir,
            ref client_extras_dir,
        } = rpc
        {
            assert!(self.is_waiting(), "client_started can only be sent once");
            let state = CoreState::new(ctx.get_peer());
            let state = Arc::new(Mutex::new(state));
            *self = Ulsp::Running(state);
            let weak_self = self.weak_self().unwrap();
            self.inner().finish_setup(weak_self);
        }

        self.inner().client_notification(rpc);
    }

    fn handle_request(&mut self, ctx: &RpcCtx, rpc: Self::Request) -> Result<Value, RemoteError> {
        self.inner().client_request(rpc)
    }

    fn idle(&mut self, _ctx: &RpcCtx, token: usize) {
        self.inner().handle_idle(token);
    }
}
