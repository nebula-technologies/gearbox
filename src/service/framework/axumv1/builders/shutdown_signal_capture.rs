use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

pub(crate) async fn shutdown_signal_capture(
    shutdown_triggered_clone: Arc<AtomicBool>,
    shutdown_notify_clone: Arc<Notify>,
) {
    let ctrl_c_count = Arc::new(AtomicUsize::new(0));

    let ctrl_c = {
        let ctrl_c_count = Arc::clone(&ctrl_c_count);
        let shutdown_triggered_clone = Arc::clone(&shutdown_triggered_clone);
        let shutdown_notify_clone = Arc::clone(&shutdown_notify_clone);

        async move {
            loop {
                signal::ctrl_c()
                    .await
                    .expect("failed to install Ctrl+C handler");

                // Increment the Ctrl+C count
                let count = ctrl_c_count.fetch_add(1, Ordering::SeqCst) + 1;
                if count == 1 {
                    // First Ctrl+C, initiate graceful shutdown
                    println!("Received Ctrl+C signal. Initiating graceful shutdown...");
                    shutdown_triggered_clone.store(true, Ordering::SeqCst); // Set the shutdown flag
                    shutdown_notify_clone.notify_one();
                } else if count >= 3 {
                    // On third Ctrl+C, force shutdown
                    println!("Received 3 Ctrl+C signals, forcing shutdown.");
                    std::process::exit(0);
                } else {
                    println!(
                        "Received Ctrl+C signal again. Press {} more time(s) to force quit.",
                        3 - count
                    );
                }
            }
        }
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
