use std::time::Duration;

use clap::Parser;
use cli::{Cli, Commands};
use tokio;

mod cli;
mod stream;

fn main() {
    let cli = Cli::parse();
    let runtime = tokio::runtime::Runtime::new().unwrap();

    match &cli.command {
        Commands::Connect { host, port, .. } => {
            println!("connecting to {}:{}", host, port);

            runtime.block_on(async {
                tokio::select! {
                    _ = stream::client() => {}
                    _ = tokio::signal::ctrl_c() => {}
                }
            })
        }

        Commands::Serve {
            bind_host, port, ..
        } => {
            println!("connecting to {}:{}", bind_host, port);

            runtime.block_on(async {
                tokio::select! {
                    _ = stream::server() => {}
                    _ = tokio::signal::ctrl_c() => {}
                }
            })
        }
    }

    runtime.shutdown_timeout(Duration::from_secs(0));
}
