use crate::service::framework::axum::builders::shutdown_signal_capture::shutdown_signal_capture;
use crate::service::framework::axum::builders::HyperConfig;
use crate::service::framework::axum::connection::builder::ConnectionBuilder;
use crate::service::framework::axum::status::{ServerRuntimeError, ServerRuntimeStatus};
use crate::{error, info, notice};
use axum::Router;
use hyper_util::rt::TokioIo;
use hyper_util::service::TowerToHyperService;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::Notify;
use tokio::time::sleep;

pub(crate) async fn spin_server<H: Into<ConnectionBuilder>>(
    http_handler_into: H,
    listener: TcpListener,
    hyper_config: HyperConfig,
    app: Router,
) -> Result<ServerRuntimeStatus, ServerRuntimeError> {
    let http_handler = http_handler_into.into();
    notice!("Starting server");

    // Atomic counter to track active connections
    let active_connections = Arc::new(AtomicUsize::new(0));
    let shutdown_notify = Arc::new(Notify::new());
    let shutdown_triggered = Arc::new(AtomicBool::new(false));

    let shutdown_notify_clone = shutdown_notify.clone();
    let shutdown_triggered_clone = shutdown_triggered.clone();

    // Spawn a task to capture shutdown signals (SIGINT or SIGTERM)
    tokio::spawn(async move {
        shutdown_signal_capture(shutdown_triggered_clone, shutdown_notify_clone).await;
    });

    let shutdown_triggered_clone = shutdown_triggered.clone();

    let service_app = TowerToHyperService::new(app);

    // Server accept loop
    loop {
        tokio::select! {
            // Accept new connections
            Ok((stream, _addr)) = listener.accept() => {
                if shutdown_triggered_clone.load(Ordering::SeqCst) {
                    error!("Server is awaiting shutdown, no new connections allowed");
                    continue;
                }
                let active_conn_clone = active_connections.clone();
                let io = TokioIo::new(stream);
                let conn = http_handler.serve_connection(io, service_app.clone());
                tokio::spawn(async move {
                    // Increment the active connection count
                    active_conn_clone.fetch_add(1, Ordering::SeqCst);
                    notice!("Accepted new connection. Active connections: {}", active_conn_clone.load(Ordering::SeqCst));


                    if let Err(e) = conn.await {
                        error!("Error serving connection: {:?}", e);
                    }
                    // Decrement the active connection count when the connection is finished
                    active_conn_clone.fetch_sub(1, Ordering::SeqCst);
                    notice!("Connection closed. Active connections: {}", active_conn_clone.load(Ordering::SeqCst));
                });
            },
            // Shutdown signal received, wait for it asynchronously
            _ = shutdown_notify.notified() => {
                notice!("Shutdown signal received.");
                break;
            }
        }
    }

    // Wait for all active connections to finish
    notice!("Waiting for active connections to finish...");
    while active_connections.load(Ordering::SeqCst) > 0 {
        sleep(Duration::from_secs(1)).await;
    }

    notice!("All connections closed, shutting down gracefully.");
    Ok(ServerRuntimeStatus::GracefulShutdown)
}
