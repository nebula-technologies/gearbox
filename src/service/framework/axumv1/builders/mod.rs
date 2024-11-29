use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub(crate) async fn spin_h2c_server(
    listener: TcpListener,
    hyper_config: HyperConfig, // Placeholder, can be expanded
    app: Router,
) -> Result<ServerRuntimeStatus, ServerRuntimeError> {
    let h2 = http2::Builder::new(TokioExecutor);
    spin_server(h2, listener, hyper_config, app).await
}

pub(crate) async fn spin_http1_server(
    listener: TcpListener,
    hyper_config: HyperConfig, // Not used in this example, but you can expand this struct
    app: Router,
) -> Result<ServerRuntimeStatus, ServerRuntimeError> {
    let http = http1::Builder::new();
    spin_server(http, listener, hyper_config, app).await
}
