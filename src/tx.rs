#![allow(dead_code)]

use crate::{
    chain::*,
    types::{MsgCreateStaker, MsgSubmitProof},
    wallet::Wallet,
};
use cosmos_sdk_proto::cosmos::{
    base::abci::v1beta1::TxResponse,
    tx::v1beta1::{service_client::ServiceClient, BroadcastMode, BroadcastTxRequest, BroadcastTxResponse, GetTxRequest},
};
use cosmrs::{
    tx::{BodyBuilder, Fee, Msg, Raw, SignDoc, SignerInfo},
    AccountId, Any, Coin, Denom, ErrorReport, Result,
};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct TxClient {
    pub wallet: Wallet,
    pub rpc: String,
    pub fee: u128,
    pub gas_limit: u64,
}

impl TxClient {
    pub fn new(private_key: &str, rpc: &str, fee: u128, gas_limit: u64) -> Self {
        let wallet = Wallet::new(private_key);
        Self {
            wallet,
            rpc: rpc.to_string(),
            fee,
            gas_limit,
        }
    }

    pub fn account_id(&self) -> AccountId {
        self.wallet.account_id.clone()
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

    // TODO: This use ServiceClient, but do not need private_key, fee, gas_limit, refactor it!
    pub async fn get_tx(&self, tx_id: &str) -> Result<TxResponse> {
        let mut client = ServiceClient::connect(self.rpc.clone()).await?;
        let resp = client
            .get_tx(GetTxRequest {
                hash: tx_id.to_string(),
            })
            .await?;
        let tx_response = resp
            .get_ref()
            .clone()
            .tx_response
            .ok_or(ErrorReport::msg("Failed to parse TxResponse while get_tx"))?;
        Ok(tx_response)
    }
}

#[cfg(test)]
mod tests {
    use super::{MsgCreateStaker, MsgSubmitProof};
    use crate::{tx::TxClient, wallet::Wallet};
    use cosmrs::AccountId;
    use sha2::{Digest, Sha256};

    const BITVM_PROOF_SYSTEM: &str = "GROTH16_BN254_BITVM";
    const TEST_DATA: &str = "test-data";
    const SENDER_PRIVATE_KEY: &str =
        "d25da18393e1e3be637cba299ff08621757a134911029abd8a52fb4a989d2f73";
    const NODE: &str = "http://13.231.104.23:9090";
    // const NODE: &str = "https://testnet-grpc.fiammachain.io";
    // grpcurl -v -d '{"address":"fiamma19fldhw0awjv2ag7dz0lr3d4qmnfkxz69rzxcdp"}' testnet-grpc.fiammachain.io:443 cosmos.auth.v1beta1.Query/Account
    // fiammad query tx --type=hash 04DD64900B9AB19D2FFB5EE0118BC4C96E3B5F44110E329412BD5EF8B722FADD --node tcp://13.231.104.23:26657 --chain-id fiamma-testnet-1

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
    async fn test_submit_proof() {
        let wallet = Wallet::new(SENDER_PRIVATE_KEY);
        let gas_limit = 80_000_000_u64;
        let fee = 2000_u128;
        let tx_client = TxClient::new(SENDER_PRIVATE_KEY, NODE, fee, gas_limit);
        let submit_proof_msg = msg_submit_proof(wallet.account_id.clone());
        let resp = tx_client.submit_proof(submit_proof_msg).await.unwrap();
        println!("resp: {:?}", resp);
    }

    #[tokio::test]
    async fn test_create_staker() {
        let wallet = Wallet::new(SENDER_PRIVATE_KEY);
        let gas_limit = 80_000_000_u64;
        let fee = 2000_u128;
        let tx_client = TxClient::new(SENDER_PRIVATE_KEY, NODE, fee, gas_limit);
        let msg = MsgCreateStaker {
            creator: wallet.account_id.clone(),
            staker_address: "fiammavaloper19fldhw0awjv2ag7dz0lr3d4qmnfkxz69vukt7x".to_string(),
        };
        let resp = tx_client.create_staker(msg).await;
        println!("resp: {:?}", resp);
    }

    #[tokio::test]
    async fn test_get_tx() {
        let gas_limit = 80_000_000_u64;
        let fee = 2000_u128;
        let tx_id = "DAFD01EB130B6497502920CA22FDC19E0898E2130748095F2579535095EF2539";
        let query_client = TxClient::new(SENDER_PRIVATE_KEY, NODE, fee, gas_limit);
        let tx = query_client.get_tx(tx_id).await;
        println!("tx: {:?}", tx);
    }
}
