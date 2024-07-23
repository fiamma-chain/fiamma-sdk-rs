#![allow(dead_code)]

use crate::{
    chain::*,
    types::{MsgCreateStaker, MsgSubmitProof},
    wallet::Wallet,
};
use cosmos_sdk_proto::cosmos::tx::v1beta1::{
    service_client::ServiceClient, BroadcastMode, BroadcastTxRequest, BroadcastTxResponse,
};
use cosmrs::{
    tx::{BodyBuilder, Fee, Msg, Raw, SignDoc, SignerInfo},
    Any, Coin, Denom, ErrorReport, Result,
};
use std::str::FromStr;

struct TxClient {
    pub wallet: Wallet,
    pub rpc: String,
    pub fee: u128,
    pub gas_limit: u64,
}

impl TxClient {
    pub fn new(wallet: Wallet, rpc: String, fee: u128, gas_limit: u64) -> Self {
        Self {
            wallet,
            rpc,
            fee,
            gas_limit,
        }
    }

    pub async fn submit_proof(&self, msg: MsgSubmitProof) -> Result<BroadcastTxResponse> {
        self.construct_broadcast_tx(msg.to_any()?).await
    }

    pub async fn create_staker(&self, msg: MsgCreateStaker) -> Result<BroadcastTxResponse> {
        self.construct_broadcast_tx(msg.to_any()?).await
    }

    async fn construct_broadcast_tx(&self, msg: impl Into<Any>) -> Result<BroadcastTxResponse> {
        let raw_tx = self.construct_tx(msg).await?;
        let mut client = ServiceClient::connect(self.rpc.clone()).await?;
        let tx_commit_response = client
            .broadcast_tx(BroadcastTxRequest {
                tx_bytes: raw_tx.to_bytes()?,
                mode: BroadcastMode::Sync as i32,
            })
            .await;

        tx_commit_response
            .map(|resp| resp.get_ref().clone())
            .map_err(ErrorReport::from)
    }

    async fn construct_tx(&self, msg: impl Into<Any>) -> Result<Raw> {
        let account = self.wallet.get_account_info(self.rpc.clone()).await;
        let (account_number, sequence) =
            account.map_or((0, 0), |acc| (acc.account_number, acc.sequence));

        let chain_id = CHAIN_ID.parse()?;
        let fee = Coin {
            amount: self.fee,
            denom: Denom::from_str(DENOM)?,
        };
        let fee = Fee::from_amount_and_gas(fee, self.gas_limit);
        let tx_body = BodyBuilder::new().msg(msg).finish();
        let auth_info =
            SignerInfo::single_direct(Some(self.wallet.public_key), sequence).auth_info(fee);
        let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_number)?;
        self.wallet.sign(sign_doc)
    }
}

#[cfg(test)]
mod tests {
    use super::MsgSubmitProof;
    use crate::{chain::*, tx::TxClient, wallet::Wallet};
    use cosmos_sdk_proto::cosmos::{
        auth::v1beta1::{query_client::QueryClient, BaseAccount, QueryAccountRequest},
        tx::v1beta1::{service_client::ServiceClient, BroadcastMode, BroadcastTxRequest},
    };
    use cosmrs::{
        crypto::secp256k1,
        tx::{self, Fee, Msg, SignDoc, SignerInfo},
        AccountId, Coin,
    };
    use sha2::{Digest, Sha256};

    const BITVM_PROOF_SYSTEM: &str = "GROTH16_BN254_BITVM";
    const TEST_DATA: &str = "test-data";
    const SENDER_PRIVATE_KEY: &str =
        "424a0d5ff7c1c9ce116c2e4cc02f0e6c1beea5507f5828aefa5453b30cae52c1";
    const NODE: &str = "https://testnet-grpc.fiammachain.io";
    // grpcurl -v -d '{"address":"fiamma19fldhw0awjv2ag7dz0lr3d4qmnfkxz69rzxcdp"}' testnet-grpc.fiammachain.io:443 cosmos.auth.v1beta1.Query/Account

    fn proof_artifacts() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        let location = std::env::current_dir().unwrap().join(TEST_DATA);

        let proof_file = location.join("proof.bitvm");
        let proof = std::fs::read(&proof_file).unwrap();

        let public_input_file = location.join("public_input.bitvm");
        let public_input = std::fs::read(&public_input_file).unwrap();

        let vk_file = location.join("vk.bitvm");
        let vk = std::fs::read(&vk_file).unwrap();
        (proof, public_input, vk)
    }

    #[test]
    fn proof_id() {
        let (proof, public_input, vk) = proof_artifacts();
        let all_data: Vec<u8> = vec![Vec::from(BITVM_PROOF_SYSTEM), proof, public_input, vk]
            .into_iter()
            .flatten()
            .collect();

        let mut hasher = Sha256::new();
        hasher.update(all_data);
        let result = hasher.finalize();
        let proof_id = hex::encode(result);
        println!("proof_id: {}", proof_id);
    }

    fn msg_submit_proof(account_id: AccountId) -> MsgSubmitProof {
        let (proof, public_input, vk) = proof_artifacts();

        MsgSubmitProof {
            creator: account_id,
            proof_system: BITVM_PROOF_SYSTEM.to_string(),
            proof,
            public_input,
            vk,
        }
    }

    #[tokio::test]
    async fn test_tx() {
        let wallet = Wallet::new(SENDER_PRIVATE_KEY);
        let gas_limit = 80_000_000_u64;
        let fee = 2000_u128;
        let tx_client = TxClient::new(wallet.clone(), NODE.to_string(), fee, gas_limit);
        let submit_proof_msg = msg_submit_proof(wallet.account_id.clone());
        let resp = tx_client.submit_proof(submit_proof_msg).await.unwrap();
        println!("resp: {:?}", resp);
    }

    #[tokio::test]
    async fn test_submit_proof_demo() {
        let sender_private_key = hex::decode(SENDER_PRIVATE_KEY).unwrap();
        let sender_private_key = secp256k1::SigningKey::from_slice(&sender_private_key).unwrap();
        let sender_public_key = sender_private_key.public_key();
        let sender_account_id = sender_public_key.account_id(ACCOUNT_PREFIX).unwrap();

        let submit_proof_msg = msg_submit_proof(sender_account_id.clone())
            .to_any()
            .unwrap();

        let mut client = QueryClient::connect(NODE).await.unwrap();
        let account_info = client
            .account(QueryAccountRequest {
                address: sender_account_id.as_ref().to_string(),
            })
            .await;

        let account_info = account_info.unwrap().get_ref().clone().account.unwrap();
        let account: BaseAccount = account_info
            .to_msg::<BaseAccount>()
            .unwrap()
            .try_into()
            .unwrap();
        println!("base_account: {:?}", account);

        let chain_id = CHAIN_ID.parse().unwrap();
        let sequence_number = account.sequence;
        let account_number = account.account_number;
        let gas = 80_000_000u64;
        let fee = Coin {
            amount: 2000,
            denom: DENOM.parse().unwrap(),
        };
        let fee = Fee::from_amount_and_gas(fee, gas);
        let tx_body = tx::BodyBuilder::new().msg(submit_proof_msg).finish();
        let auth_info =
            SignerInfo::single_direct(Some(sender_public_key), sequence_number).auth_info(fee);
        let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_number).unwrap();
        let tx_raw = sign_doc.sign(&sender_private_key).unwrap();

        let mut client = ServiceClient::connect(NODE).await.unwrap();
        let tx_commit_response = client
            .broadcast_tx(BroadcastTxRequest {
                tx_bytes: tx_raw.to_bytes().unwrap(),
                mode: BroadcastMode::Sync as i32,
            })
            .await;

        println!("tx_commit_response: {:?}", tx_commit_response);
    }
}
