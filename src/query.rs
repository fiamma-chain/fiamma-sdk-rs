#![allow(dead_code)]

use crate::generated::fiamma::zkpverify::{
    query_client::QueryClient as ProtoQueryClient, BitVmChallengeData, ProofData,
    QueryBitVmChallengeDataRequest, QueryProofDataRequest,
};
use cosmos_sdk_proto::cosmos::{
    base::abci::v1beta1::TxResponse,
    tx::v1beta1::{service_client::ServiceClient, GetTxRequest},
};
use cosmrs::{ErrorReport, Result};

pub struct QueryClient {
    rpc: String,
}

impl QueryClient {
    pub fn new(rpc: String) -> Self {
        Self { rpc }
    }

    pub async fn get_proof_data(&self, proof_id: &str) -> Result<ProofData> {
        let mut client = ProtoQueryClient::connect(self.rpc.clone()).await?;
        let resp = client
            .proof_data(QueryProofDataRequest {
                proof_id: proof_id.to_string(),
            })
            .await?;
        let proof_data = resp
            .get_ref()
            .clone()
            .proof_data
            .ok_or(ErrorReport::msg("Empty proof data in response"))?;
        Ok(proof_data)
    }

    pub async fn get_bitvm_challenge_data(&self, proof_id: &str) -> Result<BitVmChallengeData> {
        let mut client = ProtoQueryClient::connect(self.rpc.clone()).await?;
        let resp = client
            .bit_vm_challenge_data(QueryBitVmChallengeDataRequest {
                proof_id: proof_id.to_string(),
            })
            .await?;
        let bitvm_challenge_data = resp
            .get_ref()
            .clone()
            .bitvm_challenge_data
            .ok_or(ErrorReport::msg("Empty bitvm challenge data in response"))?;
        Ok(bitvm_challenge_data)
    }

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
    use super::QueryClient;
    const NODE: &str = "http://13.231.104.23:9090";

    #[tokio::test]
    async fn test_get_proof_data() {
        let proof_id = "00e2af0c74cf8091cf1fd60c672698be7700a5ddfd1d94c21ec06df5bf82da80";
        let query_client = QueryClient::new(NODE.to_string());
        let proof_data = query_client.get_proof_data(proof_id).await;
        println!("proof_data: {:?}", proof_data);
    }

    #[tokio::test]
    async fn test_get_bitvm_challenge_data() {
        let proof_id = "00e2af0c74cf8091cf1fd60c672698be7700a5ddfd1d94c21ec06df5bf82da80";
        let query_client = QueryClient::new(NODE.to_string());
        let bitvm_challenge_data = query_client.get_bitvm_challenge_data(proof_id).await;
        println!("bitvm_challenge_data: {:?}", bitvm_challenge_data);
    }

    #[tokio::test]
    async fn test_get_tx() {
        let tx_id = "EB65C148A8A06B0F0E36967E27E38B6AE6D40C1DCB6D7B12F7B305729D373AB5";
        let query_client = QueryClient::new(NODE.to_string());
        let tx = query_client.get_tx(tx_id).await;
        println!("tx: {:?}", tx);
    }
}
