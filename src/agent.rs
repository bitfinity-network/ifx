use ic_agent::{
    agent::http_transport::ReqwestHttpReplicaV2Transport, identity::BasicIdentity, Agent,
    AgentError,
};

pub async fn create_agent(pem_path: &str, network: &str) -> Result<Agent, AgentError> {
    let identity = BasicIdentity::from_pem_file(pem_path).unwrap();
    let agent = Agent::builder().with_identity(identity);

    match network {
        "ic" => {
            let url = "https://ic0.app";
            let agent = agent
                .with_transport(ReqwestHttpReplicaV2Transport::create(url)?)
                .build()?;

            Ok(agent)
        }
        n => {
            let mut url = n;
            if let "local" = n {
                url = "http://localhost:8000";
            }

            let agent = agent
                .with_transport(ReqwestHttpReplicaV2Transport::create(url)?)
                .build()?;

            agent.fetch_root_key().await.unwrap();
            Ok(agent)
        }
    }
}
