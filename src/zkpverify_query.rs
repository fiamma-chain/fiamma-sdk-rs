#![allow(dead_code)]

use crate::generated::fiamma::zkpverify::{
    query_client::QueryClient as ProtoQueryClient, BitVmChallengeData, DaSubmissionData,
    DaSubmissionResult, ProofData, QueryBitVmChallengeDataRequest, QueryDaSubmissionDataRequest,
    QueryDaSubmissionResultRequest, QueryPendingProofByNamespaceRequest, QueryPendingProofRequest,
    QueryProofDataRequest, QueryVerifyResultRequest, QueryVerifyResultsByNamespaceRequest,
    VerifyResult,
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

    pub async fn get_verify_result_by_namespace(
        &self,
        namespace: &str,
    ) -> Result<Vec<VerifyResult>> {
        let mut client = ProtoQueryClient::connect(self.rpc.clone()).await?;
        let resp = client
            .verify_results_by_namespace(QueryVerifyResultsByNamespaceRequest {
                namespace: namespace.to_string(),
                pagination: None,
            })
            .await?;
        let verify_results = resp.get_ref().clone().verify_results;
        if verify_results.is_empty() {
            return Err(ErrorReport::msg(
                "Empty verify result data from namespace in response",
            ));
        }
        Ok(verify_results)
    }

    pub async fn get_pending_proof(&self) -> Result<Vec<VerifyResult>> {
        let mut client = ProtoQueryClient::connect(self.rpc.clone()).await?;
        let resp = client
            .pending_proof(QueryPendingProofRequest { pagination: None })
            .await?;
        let pending_proofs = resp.get_ref().clone().pending_proofs;
        if pending_proofs.is_empty() {
            return Err(ErrorReport::msg("Empty pending proofs in response"));
        }
        Ok(pending_proofs)
    }

    pub async fn get_pending_proof_by_namespace(
        &self,
        namespace: &str,
    ) -> Result<Vec<VerifyResult>> {
        let mut client = ProtoQueryClient::connect(self.rpc.clone()).await?;
        let resp = client
            .pending_proof_by_namespace(QueryPendingProofByNamespaceRequest {
                namespace: namespace.to_string(),
                pagination: None,
            })
            .await?;
        let pending_proofs = resp.get_ref().clone().pending_proofs;
        if pending_proofs.is_empty() {
            return Err(ErrorReport::msg(
                "Empty pending proofs by namespace in response",
            ));
        }
        Ok(pending_proofs)
    }

    pub async fn get_da_submission_data(&self, proof_id: &str) -> Result<DaSubmissionData> {
        let mut client = ProtoQueryClient::connect(self.rpc.clone()).await?;
        let resp = client
            .da_submission_data(QueryDaSubmissionDataRequest {
                proof_id: proof_id.to_string(),
            })
            .await?;
        let da_submission_data = resp
            .get_ref()
            .clone()
            .da_submission_data
            .ok_or(ErrorReport::msg("Empty da submission data in response"))?;
        Ok(da_submission_data)
    }

    pub async fn get_da_submission_result(&self, proof_id: &str) -> Result<DaSubmissionResult> {
        let mut client = ProtoQueryClient::connect(self.rpc.clone()).await?;
        let resp = client
            .da_submission_result(QueryDaSubmissionResultRequest {
                proof_id: proof_id.to_string(),
            })
            .await?;
        let da_submission_result = resp
            .get_ref()
            .clone()
            .da_submission_result
            .ok_or(ErrorReport::msg("Empty da submission result in response"))?;
        Ok(da_submission_result)
    }
}

#[cfg(test)]
mod tests {
    use super::QueryClient;
    const NODE: &str = "http://54.65.75.57:9090";
    const NAMESPACE: &str = "test-namespace";

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
        println!("get_verify_result: {:?}", get_verify_result);
    }

    #[tokio::test]
    async fn test_get_verify_result_by_namespace() {
        let query_client = QueryClient::new(NODE);
        let get_verify_results = query_client.get_verify_result_by_namespace(NAMESPACE).await;
        println!("get_verify_result_by_namespace: {:?}", get_verify_results);
    }

    #[tokio::test]
    async fn test_get_pending_proof() {
        let query_client = QueryClient::new(NODE);
        let get_pending_proof = query_client.get_pending_proof().await;
        println!("get_pending_proof: {:?}", get_pending_proof);
    }

    #[tokio::test]
    async fn test_get_pending_proof_by_namespace() {
        let query_client = QueryClient::new(NODE);
        let get_pending_proof_by_namespace =
            query_client.get_pending_proof_by_namespace(NAMESPACE).await;
        println!(
            "get_pending_proof_by_namespace: {:?}",
            get_pending_proof_by_namespace
        );
    }
}
