#![allow(dead_code)]

use crate::generated::fiamma::zkpverify::{
    query_client::QueryClient as ProtoQueryClient, ProofData, QueryProofDataRequest,
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
}

#[cfg(test)]
mod tests {
    use super::QueryClient;
    const NODE: &str = "https://testnet-grpc.fiammachain.io";

    #[tokio::test]
    async fn test_get_proof_data() {
        let proof_id = "00e2af0c74cf8091cf1fd60c672698be7700a5ddfd1d94c21ec06df5bf82da80";
        let query_client = QueryClient::new(NODE.to_string());
        let proof_data = query_client.get_proof_data(proof_id).await;
        println!("proof_data: {:?}", proof_data);
    }
}
