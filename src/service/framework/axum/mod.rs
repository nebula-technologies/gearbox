pub mod connection;
pub mod connection_builder;
pub mod discovery_builder;
pub mod executor;
pub mod framework_state;
pub mod hyper_config;
pub mod log_formatter;
pub mod log_output_style;
pub mod module_manager;
pub mod server_builder;
pub mod status;

pub use self::{
    connection::*, connection_builder::*, discovery_builder::*, executor::*, framework_state::*,
    hyper_config::*, log_formatter::*, log_output_style::*, module_manager::*, server_builder::*,
    status::*,
};

use std::sync::Arc;

pub type BoxFn<T> = Arc<dyn Fn() -> T + Send + Sync>;

#[cfg(test)]
mod test {
    use super::{FrameworkState, ModuleDefinition, RwFrameworkState};
    use axum::Router;
    use std::sync::Arc;

    pub struct TestState {}
    pub struct TestModule {}

    impl TestModule {
        pub fn routes() -> Router<Arc<FrameworkState>> {
            Router::new()
        }
        pub fn states(state: &mut RwFrameworkState) {
            state.add(TestState {});
        }
    }

    impl ModuleDefinition for TestModule {
        const NAME: &'static str = "TestModule";
        const ROUTER: fn() -> Router<Arc<FrameworkState>> = TestModule::routes;
        const STATES: fn(&mut RwFrameworkState) = TestModule::states;
    }

    #[test]
    fn setup_builder() {
        //
        // fn audit_module(m: ModuleBuilder)
        //
        // let mut state = AppState::default();
        // state.add::<crate::AppState>();
        //
        // let router = Router::new()
        //     .nest("/", modules::audit_log::routes())
        //     .with_state(Arc::new(state.build()));

        let server_builder = crate::service::framework::axum::ServerBuilder::new();
        server_builder
            .add_module::<TestModule>()
            .set_worker_pool(30)
            .with_log_service_discovery(|t| None)
            .build_test();
    }
}
