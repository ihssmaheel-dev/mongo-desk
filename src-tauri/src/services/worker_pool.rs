use std::collections::HashMap;

use crate::error::AppError;
use tokio_util::sync::CancellationToken;

pub struct WorkerPool {
    workers: HashMap<String, CancellationToken>,
    max_workers: usize,
}

impl WorkerPool {
    pub fn new(max_workers: usize) -> Self {
        Self {
            workers: HashMap::new(),
            max_workers,
        }
    }

    pub fn spawn_worker<F>(
        &mut self,
        id: String,
        task: F,
    ) -> Result<(), AppError>
    where
        F: std::future::Future<Output = Result<(), AppError>> + Send + 'static,
    {
        if self.workers.len() >= self.max_workers {
            return Err(AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Worker pool is full.".into(),
            });
        }

        let token = CancellationToken::new();
        let worker_token = token.clone();
        let worker_id = id.clone();

        tokio::spawn(async move {
            tokio::select! {
                result = task => {
                    if let Err(e) = result {
                        tracing::error!("Worker {} completed with error: {:?}", worker_id, e);
                    }
                }
                _ = worker_token.cancelled() => {
                    tracing::info!("Worker {} cancelled", worker_id);
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
