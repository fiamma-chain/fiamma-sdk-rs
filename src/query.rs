#![allow(dead_code)]

use crate::generated::fiamma::zkpverify::{
    query_client::QueryClient as ProtoQueryClient, BitVmChallengeData, ProofData,
    QueryBitVmChallengeDataRequest, QueryProofDataRequest, QueryVerifyResultRequest, VerifyResult,
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

    pub async fn get_verify_result(&self, proof_id: &str) -> Result<VerifyResult> {
        let mut client = ProtoQueryClient::connect(self.rpc.clone()).await?;
        let resp = client
            .verify_result(QueryVerifyResultRequest {
                proof_id: proof_id.to_string(),
            })
            .await?;
        let verify_result = resp
            .get_ref()
            .clone()
            .verify_result
            .ok_or(ErrorReport::msg("Empty verify result data in response"))?;
        Ok(verify_result)
    }
}

#[cfg(test)]
mod tests {
    use super::QueryClient;
    const NODE: &str = "http://54.65.75.57:9090";

    #[tokio::test]
    async fn test_get_proof_data() {
        let proof_id = "8a17276c37500fe1f0b277f21205592eac037b60f8a7021713ed2b99fe4f78f2";
        let query_client = QueryClient::new(NODE);
        let proof_data = query_client.get_proof_data(proof_id).await;
        println!("proof_data: {:?}", proof_data);
    }

    #[tokio::test]
    async fn test_get_bitvm_challenge_data() {
        let proof_id = "8a17276c37500fe1f0b277f21205592eac037b60f8a7021713ed2b99fe4f78f2";
        let query_client = QueryClient::new(NODE);
        let bitvm_challenge_data = query_client.get_bitvm_challenge_data(proof_id).await;
        println!("bitvm_challenge_data: {:?}", bitvm_challenge_data);
    }

    #[tokio::test]
    async fn test_get_verify_result() {
        let proof_id = "8a17276c37500fe1f0b277f21205592eac037b60f8a7021713ed2b99fe4f78f2";
        let query_client = QueryClient::new(NODE);
        let get_verify_result = query_client.get_verify_result(proof_id).await;
        println!(
            "bitvm_challenget_verify_resultge_data: {:?}",
            get_verify_result
        );
    }
}
