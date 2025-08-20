use std::time::Duration;
const MAX_LISTENER_LIFETIME: u64 = 60 * 10;

pub(crate) struct PollerConfig {
    pub(crate)max_wait: Duration,
    pub(crate) refresh_interval: Duration,
}

impl Default for PollerConfig {
    fn default() -> Self {
        Self {
            max_wait: Duration::from_millis(2000),
            refresh_interval: Duration::from_millis(500),
        }
    }
}

impl PollerConfig {
    pub(crate) fn for_event_listener() -> Self {
        Self {
            max_wait: Duration::from_secs(MAX_LISTENER_LIFETIME),
            refresh_interval: Duration::from_millis(1000),
        }
    }
}