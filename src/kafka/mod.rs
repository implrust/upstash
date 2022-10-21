use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::Handler;

#[derive(Debug, Clone, Serialize)]
pub enum CreateClusterRegion {
    #[serde(rename = "us-east-1")]
    UsEast1,
    #[serde(rename = "eu-west-1")]
    EuWest1,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateClusterRequest {
    pub name: String,
    pub region: CreateClusterRegion,
    pub multizone: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct RenameClusterRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateTopicCleanupPolicy {
    Compact,
    Delete,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateTopicRequest {
    pub name: String,
    pub partitions: u32,
    pub retention_time: u32,
    pub retention_size: u32,
    pub max_message_size: u32,
    pub cleanup_policy: CreateTopicCleanupPolicy,
    pub cluster_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReconfigureTopicRequest {
    pub retention_time: Option<u32>,
    pub retention_size: Option<u32>,
    pub max_message_size: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TopicResponse {
    pub topic_id: String,
    pub topic_name: String,
    pub cluster_id: String,
    pub region: String,
    pub creation_time: usize,
    pub state: String,
    pub partitions: u32,
    pub multizone: Option<bool>,
    pub tcp_endpoint: String,
    pub rest_endpoint: String,
    pub username: String,
    pub password: String,
    pub cleanup_policy: String,
    pub retention_size: u32,
    pub retention_time: u32,
    pub max_message_size: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClusterResponse {
    pub cluster_id: String,
    pub name: String,
    pub region: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub multizone: Option<bool>,
    pub tcp_endpoint: String,
    pub rest_endpoint: String,
    pub state: String,
    pub username: String,
    pub password: String,
    pub max_retention_size: usize,
    pub max_retention_time: usize,
    pub max_messages_per_second: u32,
    pub creation_time: usize,
    pub max_message_size: u32,
    pub max_partitions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CredentialState {
    Active,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CredentialPermissions {
    All,
    Produce,
    Consume,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCredentialRequest {
    pub credential_name: String,
    pub topic: String,
    pub permissions: CredentialPermissions,
    pub cluster_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CredentialResponse {
    pub credential_id: String,
    pub credential_name: String,
    pub topic: String,
    pub permissions: CredentialPermissions,
    pub cluster_id: String,
    pub username: String,
    pub creation_time: usize,
    pub state: CredentialState,
    pub password: String,
    pub encoded_username: String,
}

#[async_trait]
pub trait KafkaService {
    async fn create_cluster(&self, req: CreateClusterRequest) -> Result<ClusterResponse>;
    async fn list_clusters(&self) -> Result<Vec<ClusterResponse>>;
    async fn get_cluster(&self, id: &str) -> Result<ClusterResponse>;
    async fn rename_cluster(&self, req: RenameClusterRequest, id: &str) -> Result<ClusterResponse>;
    async fn reset_password(&self, id: &str) -> Result<ClusterResponse>;
    async fn delete_cluster(&self, id: &str) -> Result<String>;
    async fn create_topic(&self, req: CreateTopicRequest) -> Result<TopicResponse>;
    async fn get_topic(&self, id: &str) -> Result<TopicResponse>;
    async fn list_topics(&self, id: &str) -> Result<Vec<TopicResponse>>;
    async fn reconfigure_topic(&self, req: ReconfigureTopicRequest, id: &str) -> Result<TopicResponse>;
    async fn delete_topic(&self, id: &str) -> Result<String>;
    async fn create_credential(&self, req: CreateCredentialRequest) -> Result<CredentialResponse>;
    async fn list_credentials(&self) -> Result<Vec<CredentialResponse>>;
    async fn delete_credential(&self, id: &str) -> Result<String>;
}

#[async_trait]
impl<'client> KafkaService for Handler<'client> {
    async fn create_cluster(&self, req: CreateClusterRequest) -> Result<ClusterResponse> {
        let url = format!("{}/cluster", &self.url);
        self.client.post(&url, Option::None::<&()>, Some(&req)).await
    }

    async fn list_clusters(&self) -> Result<Vec<ClusterResponse>> {
        let url = format!("{}/clusters", &self.url);
        self.client.get(&url, Option::None::<&()>).await
    }

    async fn get_cluster(&self, id: &str) -> Result<ClusterResponse> {
        let url = format!("{}/cluster/{}", &self.url, id);
        self.client.get(&url, Option::None::<&()>).await
    }

    async fn rename_cluster(&self, req: RenameClusterRequest, id: &str) -> Result<ClusterResponse> {
        let url = format!("{}/rename-cluster/{}", &self.url, id);
        self.client.post(&url, Option::None::<&()>, Some(&req)).await
    }

    async fn reset_password(&self, id: &str) -> Result<ClusterResponse> {
        let url = format!("{}/reset-password/{}", &self.url, id);
        self.client.post(&url, Option::None::<&()>, Option::None::<&()>).await
    }

    async fn delete_cluster(&self, id: &str) -> Result<String> {
        let url = format!("{}/cluster/{}", &self.url, id);
        self.client.delete(&url, Option::None::<&()>).await
    }

    async fn create_topic(&self, req: CreateTopicRequest) -> Result<TopicResponse> {
        let url = format!("{}/topic", &self.url);
        self.client.post(url, Option::None::<&()>, Some(&req)).await
    }

    async fn get_topic(&self, id: &str) -> Result<TopicResponse> {
        let url = format!("{}/topic/{}", &self.url, id);
        self.client.get(&url, Option::None::<&()>).await
    }

    async fn list_topics(&self, id: &str) -> Result<Vec<TopicResponse>> {
        let url = format!("{}/topics/{}", &self.url, id);
        self.client.get(&url, Option::None::<&()>).await
    }

    async fn reconfigure_topic(&self, req: ReconfigureTopicRequest, id: &str) -> Result<TopicResponse> {
        let url = format!("{}/update-topic/{}", &self.url, id);
        self.client.post(&url, Option::None::<&()>, Some(&req)).await
    }

    async fn delete_topic(&self, id: &str) -> Result<String> {
        let url = format!("{}/topic/{}", &self.url, id);
        self.client.delete(&url, Option::None::<&()>).await
    }

    async fn create_credential(&self, req: CreateCredentialRequest) -> Result<CredentialResponse> {
        let url = format!("{}/credential ", &self.url);
        self.client.post(url, Option::None::<&()>, Some(&req)).await
    }

    async fn list_credentials(&self) -> Result<Vec<CredentialResponse>> {
        let url = format!("{}/credentials", &self.url);
        self.client.get(&url, Option::None::<&()>).await
    }

    async fn delete_credential(&self, id: &str) -> Result<String> {
        let url = format!("{}/credential/{}", &self.url, id);
        self.client.delete(&url, Option::None::<&()>).await
    }
}
