#![allow(dead_code)]

use crate::generated::fiamma::zkpverify::{
    query_client::QueryClient as ProtoQueryClient, BitVmChallengeData, ProofData,
    QueryBitVmChallengeDataRequest, QueryProofDataRequest,
};
use cosmrs::{ErrorReport, Result};

#[derive(Debug, Clone)]
pub struct QueryClient {
    rpc: String,
}

impl QueryClient {
    pub fn new(rpc: &str) -> Self {
        Self {
            rpc: rpc.to_string(),
        }
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
}

#[cfg(test)]
mod tests {
    use super::QueryClient;
    const NODE: &str = "http://54.65.137.66:9090";

    #[tokio::test]
    async fn test_get_proof_data() {
        let proof_id = "ad6d525285d38ca83ddf34b74c2728dc35cda1dcf333975117aa0e4cc7dac724";
        let query_client = QueryClient::new(NODE);
        let proof_data = query_client.get_proof_data(proof_id).await;
        println!("proof_data: {:?}", proof_data);
    }

    #[tokio::test]
    async fn test_get_bitvm_challenge_data() {
        let proof_id = "ad6d525285d38ca83ddf34b74c2728dc35cda1dcf333975117aa0e4cc7dac724";
        let query_client = QueryClient::new(NODE);
        let bitvm_challenge_data = query_client.get_bitvm_challenge_data(proof_id).await;
        println!("bitvm_challenge_data: {:?}", bitvm_challenge_data);
    }
}
