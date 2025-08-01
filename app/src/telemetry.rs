use std::io::{stdout};
use tracing::Subscriber;
use tracing::subscriber::{ set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber::fmt::MakeWriter;

// build logger subscriber
pub fn build_tracing_subscriber<W>(
    name: String,
    filter: String,
    writer: W,
) -> impl Subscriber + Send + Sync + 'static
where
    W: for<'a> MakeWriter<'a> + 'static + Send + Sync,
{
    let env_filter_layer = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(filter));

    let bunyan_formatter_std_layer = BunyanFormattingLayer::new(name.clone(), stdout);

    let bunyan_formatter_write_layer = BunyanFormattingLayer::new(name, writer);

    Registry::default()
        .with(env_filter_layer)
        // This layer is only concerned with information storage, it does not do any formatting or provide any output.
        .with(JsonStorageLayer) // JsonStorageLayer is unit struct, can be used directly
        // This layer is exclusively concerned with formatting information using the Bunyan format.
        // It relies on the upstream JsonStorageLayer to get access to the fields attached to each span.
        .with(bunyan_formatter_std_layer)
        .with(bunyan_formatter_write_layer)
}

// init logger
pub fn init_logger<S>(subscriber: S)
where
    S: Subscriber + Send + Sync + 'static,
{
    set_global_default(subscriber).expect("Failed to set subscriber");
}
