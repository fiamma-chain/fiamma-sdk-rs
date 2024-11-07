#![allow(dead_code)]

use crate::generated::fiamma::bitvmstaker::{
    query_client::QueryClient as ProtoQueryClient, QueryAllStakerInfoRequest,
    QueryCommitteeAddressRequest, QueryRegisteredVkListRequest, StakerInfo,
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

    pub async fn get_all_staker_info(&self) -> Result<Vec<StakerInfo>> {
        let mut client = ProtoQueryClient::connect(self.rpc.clone()).await?;
        let resp = client
            .all_staker_info(QueryAllStakerInfoRequest { pagination: None })
            .await?;
        let all_staker_info = resp.get_ref().clone().all_staker_info;
        if all_staker_info.is_empty() {
            return Err(ErrorReport::msg("Empty all staker info in response"));
        }
        Ok(all_staker_info)
    }

    pub async fn get_committee_address(&self) -> Result<String> {
        let mut client = ProtoQueryClient::connect(self.rpc.clone()).await?;
        let resp = client
            .committee_address(QueryCommitteeAddressRequest {})
            .await?;
        Ok(resp.get_ref().clone().committee_address)
    }

    pub async fn get_registered_vk_list(&self) -> Result<Vec<String>> {
        let mut client = ProtoQueryClient::connect(self.rpc.clone()).await?;
        let resp = client
            .registered_vk_list(QueryRegisteredVkListRequest { pagination: None })
            .await?;
        Ok(resp
            .get_ref()
            .clone()
            .registered_vk_list
            .iter()
            .map(|vk| hex::encode(vk))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::QueryClient;
    const NODE: &str = "http://54.65.75.57:9090";
    const NAMESPACE: &str = "test-namespace";

    #[tokio::test]
    async fn test_get_all_staker_info() {
        let client = QueryClient::new(NODE);
        let staker_info = client.get_all_staker_info().await.unwrap();
        println!("{:?}", staker_info);
    }

    #[tokio::test]
    async fn test_get_committee_address() {
        let client = QueryClient::new(NODE);
        let committee_address = client.get_committee_address().await.unwrap();
        println!("{}", committee_address);
    }

    #[tokio::test]
    async fn test_get_registered_vk_list() {
        let client = QueryClient::new(NODE);
        let vk_list = client.get_registered_vk_list().await.unwrap();
        println!("{:?}", vk_list);
    }
}
