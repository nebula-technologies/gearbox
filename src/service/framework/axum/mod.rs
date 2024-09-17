use crate::collections::HashMap;
use axum::body::HttpBody;
use axum::handler::Handler;
use axum::response::Response;
use axum::routing::{MethodFilter, MethodRouter};
use std::convert::Infallible;
use std::future::Future;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use tokio::sync::Semaphore;

struct ServerBuilder {
    address: String,
    dynamic_worker_pool: bool,
    worker_min: usize,
    worker_max: usize,
    worker_buffer: usize,
    worker_scaling_step: usize, // Step for scaling up and down
    semaphore: Arc<Semaphore>,
    active_tasks: Arc<AtomicUsize>,
}

impl ServerBuilder {
    fn new() -> Self {
        Self {
            address: "0.0.0.0:3000".to_string(),
            dynamic_worker_pool: false,
            worker_min: 10,
            worker_max: 192,        // Default maximum worker pool size
            worker_buffer: 4,       // Default buffer
            worker_scaling_step: 2, // Default scaling step (up or down by 2)
            semaphore: Arc::new(Semaphore::new(4)), // Start with 4 workers
            active_tasks: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn activate_dynamic_worker_pool(mut self) -> Self {
        self.dynamic_worker_pool = true;
        self
    }

    fn set_worker_min(mut self, max_workers: usize) -> Self {
        self.worker_min = max_workers;
        self
    }

    fn set_worker_max(mut self, max_workers: usize) -> Self {
        self.worker_max = max_workers;
        self
    }

    fn set_worker_buffer(mut self, buffer: usize) -> Self {
        self.worker_buffer = buffer;
        self.semaphore = Arc::new(Semaphore::new(buffer)); // Reset the semaphore with the buffer value
        self
    }

    fn set_worker_scaling_step(mut self, step: usize) -> Self {
        self.worker_scaling_step = step;
        self
    }

    fn build(self) {}
}

pub struct RouteBuilder<T, S> {
    routes: HashMap<
        String,
        HashMap<
            MethodFilter,
            Box<dyn Handler<T, S, Future = dyn Future<Output = Response> + Send + 'static>>,
        >,
    >,
}

impl<T, S, B> RouteBuilder<T, S> {
    pub fn post<H>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        B: HttpBody + Send + 'static,
        T: 'static,
        S: Clone + Send + Sync + 'static,
    {
        if self
            .routes
            .entry(path.to_string())
            .or_insert(HashMap::new())
            .insert(MethodFilter::POST, handler)
            .is_some()
        {
            println!("Override Warning: POST[{}]", path);
        }
        self
    }
}

fn main() {
    ServerBuilder::new()
        .activate_dynamic_worker_pool()
        .set_worker_max(192)
        .set_worker_buffer(8)
        .set_worker_scaling_step(4) // Set scaling step to 4
        .build();
}
