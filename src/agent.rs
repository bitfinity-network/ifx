use crate::IdentityType;
use ic_agent::{
    agent::http_transport::ReqwestHttpReplicaV2Transport, identity::{BasicIdentity, Secp256k1Identity}, Agent,
    AgentError,
};

pub async fn create_agent(
    id_type: &IdentityType,
    pem_path: &str,
    network: &str,
) -> Result<Agent, AgentError> {
    let agent =  match id_type {
        IdentityType::ED25519 => Agent::builder().with_identity(BasicIdentity::from_pem_file(pem_path).unwrap()),
        IdentityType::SECP256K1 => Agent::builder().with_identity(Secp256k1Identity::from_pem_file(pem_path).unwrap())
    };

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
