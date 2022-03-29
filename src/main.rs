mod agent;
mod wasm_upload;

use crate::agent::create_agent;
use clap::{Parser, Subcommand};
use wasm_upload::WasmUpload;

/// InfinitySwap DFINITY Executor
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// compute network to connect to
    #[clap(short, long, default_value_t = String::from("local"))]
    network: String,

    /// Path to an identity pem file
    #[clap(short, long, default_value_t = String::from("identity.pem"))]
    identity: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Upload wasm to a canister
    WasmUpload(WasmUpload),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let agent = create_agent(&cli.identity, &cli.network).await.unwrap();

    match &cli.command {
        Commands::WasmUpload(wasm_upload) => {
            wasm_upload.run(&agent).await;
        }
    }
}
