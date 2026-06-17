use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::Mutex;
use mongodb::Client;
use tokio::sync::mpsc;

use crate::error::AppError;

pub struct WorkerPool {
    workers: HashMap<String, CancellationToken>,
    max_workers: usize,
}

use tokio_util::sync::CancellationToken;

impl WorkerPool {
    pub fn new(max_workers: usize) -> Self {
        Self {
            workers: HashMap::new(),
            max_workers,
        }
    }

    pub fn spawn_worker<F, T>(
        &mut self,
        id: String,
        task: F,
        mut progress_tx: mpsc::Sender<T>,
    ) -> Result<(), AppError>
    where
        F: std::future::Future<Output = Result<(), AppError>> + Send + 'static,
        T: Send + 'static,
    {
        if self.workers.len() >= self.max_workers {
            return Err(AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Worker pool is full.".into(),
            });
        }

        let token = CancellationToken::new();
        let worker_token = token.clone();
        
        tokio::spawn(async move {
            tokio::select! {
                result = task() => {
                    if let Err(e) = tracing::error!("Worker {} completed with error: {:?}", id, e) {}
                }
                _ = worker_token.cancelled() => {
                    tracing::info!("Worker {} cancelled", id);
                }
            }
        });

        self.workers.insert(id, token);
        Ok(())
    }

    pub fn cancel_worker(&mut self, id: &str) -> bool {
        if let Some(token) = self.workers.remove(id) {
            token.cancel();
            true
        } else {
            false
        }
    }

    pub fn cancel_all(&mut self) {
        for (_, token) in self.workers.drain() {
            token.cancel();
        }
    }
}
