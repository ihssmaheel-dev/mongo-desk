use parking_lot::Mutex;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

pub struct AppState {
    pub cancellation_tokens: Arc<Mutex<CancellationTokenRegistry>>,
}

pub struct CancellationTokenRegistry {
    tokens: std::collections::HashMap<String, CancellationToken>,
}

impl CancellationTokenRegistry {
    pub fn new() -> Self {
        Self {
            tokens: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: String, token: CancellationToken) {
        self.tokens.insert(id, token);
    }

    pub fn get(&self, id: &str) -> Option<CancellationToken> {
        self.tokens.get(id).cloned()
    }

    pub fn remove(&mut self, id: &str) -> Option<CancellationToken> {
        self.tokens.remove(id)
    }

    pub fn cancel(&mut self, id: &str) -> bool {
        if let Some(token) = self.tokens.get(id) {
            token.cancel();
            true
        } else {
            false
        }
    }
}

impl AppState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            cancellation_tokens: Arc::new(Mutex::new(CancellationTokenRegistry::new())),
        })
    }
}
