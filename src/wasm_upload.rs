use clap::Args;
use ic_agent::{ic_types::Principal, Agent, AgentError};
use std::{fs, time::Duration};

#[derive(Args)]
pub struct WasmUpload {
    /// Path to the wasm you want to upload
    path: String,

    /// Path to the state header file. If not set, the file path will be considered to be the same
    /// as the `path` parameter but with `.header` extension.
    state_header_path: Option<String>,

    /// method name to call on the canister
    #[clap(long, short, default_value = "set_token_bytecode")]
    method_name: String,

    /// canister you want to upload the wasm to
    #[clap(parse(try_from_str))]
    canister_id: Principal,
}

impl WasmUpload {
    async fn upload(
        &self,
        agent: &Agent,
        wasm: &Vec<u8>,
        state_header: &Vec<u8>,
    ) -> Result<Vec<u8>, AgentError> {
        println!("Uploading wasm...");

        let waiter = garcon::Delay::builder()
            .timeout(Duration::from_secs(60 * 5))
            .build();
        let args = candid::encode_args((wasm, state_header)).unwrap();

        agent
            .update(&self.canister_id, &self.method_name)
            .with_arg(args)
            .call_and_wait(waiter)
            .await
    }

    pub async fn run(&self, agent: &Agent) {
        let wasm = fs::read(&self.path).unwrap();
        let state_header = self.read_state_header();

        self.upload(agent, &wasm, &state_header)
            .await
            .expect("failed to upload wasm");

        println!("Done!");
    }

    fn read_state_header(&self) -> Vec<u8> {
        fs::read(&self.state_header_path()).unwrap()
    }

    fn state_header_path(&self) -> std::path::PathBuf {
        self.state_header_path
            .as_ref()
            .map(|v| std::path::Path::new(v).to_path_buf())
            .unwrap_or_else(|| std::path::Path::new(&self.path).with_extension(".header"))
    }
}
