use tokio::time::Duration;
pub use nash_native_client::{Client, Environment};
use super::NashCredentials;

/// This struct represents the parameters
#[derive(Clone)]
pub struct NashParameters {
    pub credentials: Option<NashCredentials>,
    pub affiliate_code: Option<String>,
    pub turn_off_sign_states: bool,
    pub sign_states_loop_interval: Option<Duration>,
    pub fill_pool_loop_interval: Option<Duration>,
    pub fill_pool_loop_blockchains: Option<Vec<Blockchain>>,
    pub client_id: u64,
    pub environment: Environment,
    pub timeout: Duration,
}
