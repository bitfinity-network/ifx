use clap::Args;
use ic_base_types::PrincipalId;
use ledger_canister::{AccountIdentifier, Subaccount};

#[derive(Args)]
pub struct AccountId {
    /// generate subaccount from principal
    #[arg(value_parser)]
    subaccount_from: PrincipalId,

    /// principal to generate account-id from
    #[arg(value_parser)]
    principal_id: PrincipalId,
}

impl AccountId {
    pub fn run(&self) {
        let subaccount = Subaccount::from(&self.subaccount_from);
        let account_id = AccountIdentifier::new(self.principal_id, Some(subaccount));
        println!("{}", account_id.to_hex())
    }
}
