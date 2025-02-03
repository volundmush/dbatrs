use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc
};

use futures::future::join_all;

use tokio;
use tokio::sync::mpsc::{Sender, Receiver, channel};

use tracing::{error, info, Level};
use tracing_subscriber;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{prelude::*, fmt};

use dbatrs_shared::TotalConf;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // This check will also ensure we're in the right directory.
    let conf = Arc::new(TotalConf::set("devel")?);

    // TODO: Make this more configurable and save to the logs directory.
    // HECK, modularize it into dbatrs-shared somehow...

    // Create a formatting layer. Customize this as needed (targets, time, etc.)
    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_level(true);

    // Create the tracy profiling layer.
    let tracy_layer = tracy_full::tracing::TracyLayer;

    // Build a composite subscriber that includes the logging filter, formatted logging, and Tracy profiling.
    let subscriber = tracing_subscriber::registry()
        .with(fmt_layer)
        .with(tracy_layer);

    // Set this as the global default subscriber.
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global subscriber");

    info!("dbatrs-game starting up...");

    //let mut v = Vec::new();

    info!("Starting all tasks...");
    //join_all(v).await;

    info!("dbatrs-portal shutting down.");
    Ok(())
}