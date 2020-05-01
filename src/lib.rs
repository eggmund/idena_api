#![warn(rust_2018_idioms)]

mod error;

use serde_json::{json, Value};
use reqwest::Client;
use error::IdenaError;

// Macro used to simplify code when requesting.
macro_rules! do_request {
    ($self:expr, $json:tt) => {
        $self.request(
            json!($json)
        ).await
    }
}

/// The main API object.
pub struct IdenaAPI {
    /// The API key for your node.
    api_key: String,
    /// The host URL of your node. Usually http://localhost:9119/ if you are running the internal node of idena-desktop.
    host_url: String,
    /// The client object used to send requests to the node.
    client: Client,
}

impl IdenaAPI {
    pub fn new(api_key: &str, host_url: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
            host_url: host_url.to_owned(),

            client: Client::new(),
        }
    }

    #[inline]
    async fn request(&self, payload: Value) -> Result<Value, IdenaError> {
        let response: Value = self.client.post(&self.host_url)
            .json(&payload)
            .send()
            .await? // Wait for it to send, then receive
            .json()  // Parse as hashmap.
            .await?;
        
        // Check for error
        if response["error"] != Value::Null {
            let error_json = response["error"].to_string();
            Err(IdenaError::NodeError(error_json))
        } else {
            Ok(response["result"].clone())
        }
    }

    /// Change the API key.
    pub fn set_api_key(&mut self, new_key: &str) {
        self.api_key = new_key.to_owned();
    }

    /// List all identities (not only validated ones).
    pub async fn identities(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "id": 1,
            "method": "dna_identities",
        })
    }

    /// Show info about identity for a given address.
    pub async fn identity(&self, address: &str) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "id": 1,
            "method": "dna_identity",
            "params": [address.to_owned()]
        })
    }

    pub async fn epoch(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
            "method": "dna_epoch",
        });
    }

    pub async fn ceremony_intervals(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn address(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    /// Get the balance of an address.
    pub async fn balance(&self, address: &str) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "id": 1,
            "method": "dna_getBalance".to_owned(),
            "params": [address.to_owned()],
        })
    }

    pub async fn transaction(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn transactions(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn pending_transactions(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn kill_identity(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn go_online(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn go_offline(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn send_invite(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn activate_invite(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn fetch_flip_short_hashes(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn fetch_flip_long_hashes(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn get_flip(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn submit_short_answers(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn submit_long_answers(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn submit_flip(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    /// Send DNA from one address to another.
    pub async fn send(&self, from_address: &str, to_address: &str, amount: f64) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self.api_key,
            "id": 1,
            "method": "dna_sendTransaction",
            "params": [{"from": from_address, "to": to_address, "amount": amount}],
        })
    }

    pub async fn sync_status(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn node_version(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn import_key(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn export_key(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }

    pub async fn enode(&self) -> Result<Value, IdenaError> {
        do_request!(self, {
            "key": self._api_key,
            "id": 1,
        });
    }
}
