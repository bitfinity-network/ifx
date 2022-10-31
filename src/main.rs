mod account_id;
mod agent;
mod wasm_upload;

use crate::agent::create_agent;
use account_id::AccountId;
use clap::{Parser, Subcommand, ValueEnum};
use wasm_upload::WasmUpload;

/// InfinitySwap DFINITY Executor
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// compute network to connect to
    #[arg(short, long, default_value_t = String::from("local"))]
    network: String,

    /// Path to an identity pem file
    #[arg(short, long, default_value_t = String::from("identity.pem"))]
    identity: String,

    /// Identity type
    #[arg(short = 't', long, value_enum, default_value_t = IdentityType::ED25519)]
    idtype: IdentityType,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum IdentityType {
    ED25519,
    SECP256K1,
}

#[derive(Subcommand)]
enum Commands {
    /// Upload wasm to a canister
    WasmUpload(WasmUpload),

    /// Generate account identifier
    AccountId(AccountId),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::WasmUpload(wasm_upload) => {
            let agent = create_agent(&cli.idtype, &cli.identity, &cli.network)
                .await
                .unwrap();
            wasm_upload.run(&agent).await;
        }
        Commands::AccountId(account_id) => {
            account_id.run();
        }
    }
}
