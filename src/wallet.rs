use crate::chain::ACCOUNT_PREFIX;
use cosmos_sdk_proto::cosmos::auth::v1beta1::{
    query_client::QueryClient, BaseAccount, QueryAccountRequest,
};
use cosmrs::{
    crypto::{secp256k1::SigningKey, PublicKey},
    tx::{Raw, SignDoc},
    AccountId, Result,
};

#[derive(Debug, Clone)]
pub struct Wallet {
    private_key: Vec<u8>,
    pub public_key: PublicKey,
    pub account_id: AccountId,
}

impl Wallet {
    pub fn new(private_key: &str) -> Self {
        let private_key =
            hex::decode(private_key).expect("private key should be hex format string");
        let signing_key = SigningKey::from_slice(&private_key).expect("Parse signing_key failed");
        let public_key = signing_key.public_key();
        let account_id = public_key
            .account_id(ACCOUNT_PREFIX)
            .expect("Obtain account id from public key failed");
        Self {
            private_key,
            public_key,
            account_id,
        }
    }

    pub async fn get_account_info(&self, rpc: String) -> Option<BaseAccount> {
        let mut client = QueryClient::connect(rpc).await.ok().map(|c| c)?;
        let resp = client
            .account(QueryAccountRequest {
                address: self.account_id.as_ref().to_string(),
            })
            .await
            .ok()
            .map(|resp| resp)?;
        let account_info = resp.get_ref().clone().account?;
        let account: BaseAccount = account_info.to_msg::<BaseAccount>().ok().map(|msg| msg)?;
        Some(account)
    }

    pub fn sign(&self, sign_doc: SignDoc) -> Result<Raw> {
        let signing_key = SigningKey::from_slice(&self.private_key)?;
        sign_doc.sign(&signing_key)
    }
}

#[cfg(test)]
mod tests {
    use super::Wallet;

    const RPC: &str = "http://54.65.137.66:9090";
    const PRIVATE_KEY: &str = "59514b4e9c63b91cc9d3b6b882f1c5ee7449890c7c1116782670c71c96957897";

    #[tokio::test]
    async fn test_wallet() {
        let wallet = Wallet::new(PRIVATE_KEY);
        let account_info = wallet.get_account_info(RPC.to_string()).await;
        println!("account_info: {:?}", account_info);
    }
}
