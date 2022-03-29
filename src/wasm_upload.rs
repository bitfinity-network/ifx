use candid::encode_one;
use clap::Args;
use ic_agent::{ic_types::Principal, Agent, AgentError};
use std::{time::Duration, fs};

#[derive(Args)]
pub struct WasmUpload {
    /// Path to the wasm you want to upload
    #[clap(long, short)]
    path: String,

    /// method name to call on the canister
    #[clap(long, short, default_value = "set_token_bytecode")]
    method_name: String,

    /// canister you want to upload the wasm to
    #[clap(parse(try_from_str))]
    canister_id: Principal,
}

impl WasmUpload {
    async fn upload(&self, agent: &Agent, wasm: &Vec<u8>) -> Result<Vec<u8>, AgentError> {
        println!("Uploading wasm...");

        let waiter = garcon::Delay::builder()
            .timeout(Duration::from_secs(60 * 5))
            .build();

        agent
            .update(&self.canister_id, &self.method_name)
            .with_arg(&encode_one(&wasm).unwrap())
            .call_and_wait(waiter)
            .await
    }

    pub async fn run(&self, agent: &Agent) {
        let wasm = fs::read(&self.path).unwrap();

        self.upload(agent, &wasm)
            .await
            .expect("failed to upload wasm");

        println!("Done!");
    }
}
