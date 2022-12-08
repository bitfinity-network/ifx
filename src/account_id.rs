use clap::Args;
use ic_base_types::PrincipalId;
use ledger_canister::{AccountIdentifier, Subaccount};

#[derive(Args)]
pub struct AccountId {
    /// generate subaccount from principal
    #[arg(long, short)]
    subaccount_from: Option<PrincipalId>,

    /// principal to generate account-id from
    #[arg(value_parser)]
    principal_id: PrincipalId,
}

impl AccountId {
    pub fn run(&self) {
        let subaccount = self.subaccount_from.map(|p| Subaccount::from(&p));
        let account_id = AccountIdentifier::new(self.principal_id, subaccount);
        println!("{}", account_id.to_hex())
    }
}
