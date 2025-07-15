use tracing::Subscriber;
use tracing::subscriber::{SetGlobalDefaultError, set_global_default};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn build_tracing_subscriber(name: String) -> impl Subscriber + Send + Sync + 'static {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    Registry::default().with(env_filter)
}

pub fn init_logger<S>(subscriber: S)
where
    S: Subscriber + Send + Sync + 'static,
{
    set_global_default(subscriber).expect("Failed to set subscriber");
}
