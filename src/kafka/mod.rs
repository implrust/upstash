use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue};
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
    pub retention_time: i32,
    pub retention_size: i32,
    pub max_message_size: i32,
    pub cleanup_policy: CreateTopicCleanupPolicy,
    pub cluster_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReconfigureTopicRequest {
    pub retention_time: Option<i32>,
    pub retention_size: Option<i32>,
    pub max_message_size: Option<i32>,
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
    pub retention_size: i32,
    pub retention_time: i32,
    pub max_message_size: i32,
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
    pub max_message_size: i32,
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

#[derive(Debug, Clone, Deserialize)]
pub struct Stat {
    pub x: String,
    pub y: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClusterStats {
    pub throughput: Vec<Stat>,
    pub produce_throughput: Vec<Stat>,
    pub consume_throughput: Vec<Stat>,
    pub diskusage: Vec<Stat>,
    pub days: Vec<String>,
    pub dailyproduce: Vec<Stat>,
    pub dailyconsume: Vec<Stat>,
    pub total_monthly_storage: u64,
    pub total_monthly_billing: u64,
    pub total_monthly_produce: u64,
    pub total_monthly_consume: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TopicStats {
    pub throughput: Vec<Stat>,
    pub produce_throughput: Vec<Stat>,
    pub consume_throughput: Vec<Stat>,
    pub diskusage: Vec<Stat>,
    pub total_monthly_storage: u64,
    pub total_monthly_produce: u64,
    pub total_monthly_consume: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub topic: String,
    pub value: String,
    pub partition: u8,
    pub key: String,
}
impl Message {
    pub fn new(
        topic: impl Into<String>,
        value: impl Into<String>,
        partition: Option<u8>,
        key: Option<impl Into<String>>,
    ) -> Self {
        Self {
            topic: topic.into(),
            value: value.into(),
            partition: partition.unwrap_or(0),
            key: key.map(|k| k.into()).unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProduceResponse {
    pub topic: String,
    pub partition: u8,
    pub offset: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchRequest {
    pub topic: String,
    pub partition: u8,
    pub offset: u64,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FetchResponse {
    pub topic: String,
    pub partition: u8,
    pub offset: u64,
    pub key: String,
    pub value: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumeRequest {
    pub topic: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ConsumeResponse {
    pub key: String,
    pub offset: u64,
    pub partition: u64,
    pub timestamp: u64,
    pub topic: String,
    pub value: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitRequest {
    pub topic: String,
    pub partition: u8,
    pub offset: u64,
}
impl CommitRequest {
    pub fn new(topic: impl Into<String>, partition: u8, offset: u64) -> Self {
        Self {
            topic: topic.into(),
            partition,
            offset,
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct CommitResponse {
    pub result: String,
    pub error: String,
    pub status: u16,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Topic {
    pub topic: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ConsumerInstance {
    pub name: String,
    pub topics: Vec<Topic>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GroupInstance {
    pub name: String,
    pub instances: Vec<ConsumerInstance>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteConsumerResponse {
    pub result: String,
    pub error: String,
    pub status: u16,
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
    async fn cluster_stats(&self, id: &str) -> Result<ClusterStats>;
    async fn topic_stats(&self, id: &str) -> Result<TopicStats>;
    async fn produce(&self, req: Vec<Message>) -> Result<Vec<ProduceResponse>>;
    async fn fetch(&self, req: FetchRequest) -> Result<Vec<FetchResponse>>;
    async fn consume(&self, group: &str, consumer: &str, req: ConsumeRequest) -> Result<Vec<ConsumeResponse>>;
    async fn commit(&self, group: &str, consumer: &str, req: Vec<CommitRequest>) -> Result<CommitResponse>;
    async fn list_consumers(&self) -> Result<Vec<GroupInstance>>;
    async fn delete_consumer(&self, group: &str, consumer: &str) -> Result<DeleteConsumerResponse>;
}

#[async_trait]
impl<'client> KafkaService for Handler<'client> {
    async fn create_cluster(&self, req: CreateClusterRequest) -> Result<ClusterResponse> {
        let url = format!("{}/cluster", &self.url);
        self.client.post(&url, Option::None::<&()>, Some(&req), None).await
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
        self.client.post(&url, Option::None::<&()>, Some(&req), None).await
    }

    async fn reset_password(&self, id: &str) -> Result<ClusterResponse> {
        let url = format!("{}/reset-password/{}", &self.url, id);
        self.client
            .post(&url, Option::None::<&()>, Option::None::<&()>, None)
            .await
    }

    async fn delete_cluster(&self, id: &str) -> Result<String> {
        let url = format!("{}/cluster/{}", &self.url, id);
        self.client.delete(&url, Option::None::<&()>).await
    }

    async fn create_topic(&self, req: CreateTopicRequest) -> Result<TopicResponse> {
        let url = format!("{}/topic", &self.url);
        self.client.post(url, Option::None::<&()>, Some(&req), None).await
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
        self.client.post(&url, Option::None::<&()>, Some(&req), None).await
    }

    async fn delete_topic(&self, id: &str) -> Result<String> {
        let url = format!("{}/topic/{}", &self.url, id);
        self.client.delete(&url, Option::None::<&()>).await
    }

    async fn create_credential(&self, req: CreateCredentialRequest) -> Result<CredentialResponse> {
        let url = format!("{}/credential", &self.url);
        self.client.post(url, Option::None::<&()>, Some(&req), None).await
    }

    async fn list_credentials(&self) -> Result<Vec<CredentialResponse>> {
        let url = format!("{}/credentials", &self.url);
        self.client.get(&url, Option::None::<&()>).await
    }

    async fn delete_credential(&self, id: &str) -> Result<String> {
        let url = format!("{}/credential/{}", &self.url, id);
        self.client.delete(&url, Option::None::<&()>).await
    }

    async fn cluster_stats(&self, id: &str) -> Result<ClusterStats> {
        let url = format!("{}/stats/cluster/{}", &self.url, id);
        self.client.get(&url, Option::None::<&()>).await
    }

    async fn topic_stats(&self, id: &str) -> Result<TopicStats> {
        let url = format!("{}/stats/topic/{}", &self.url, id);
        self.client.get(&url, Option::None::<&()>).await
    }

    async fn produce(&self, req: Vec<Message>) -> Result<Vec<ProduceResponse>> {
        self.client.post(&self.url, Option::None::<&()>, Some(&req), None).await
    }

    async fn fetch(&self, req: FetchRequest) -> Result<Vec<FetchResponse>> {
        self.client.post(&self.url, Option::None::<&()>, Some(&req), None).await
    }

    async fn consume(&self, group: &str, consumer: &str, req: ConsumeRequest) -> Result<Vec<ConsumeResponse>> {
        let url = format!("{}/{}/{}", &self.url, group, consumer);
        let mut headers = HeaderMap::new();
        headers.insert("Kafka-Enable-Auto-Commit", HeaderValue::from_str("false").unwrap());
        headers.insert("Kafka-Auto-Offset-Reset", HeaderValue::from_str("latest").unwrap());
        self.client
            .post(&url, Option::None::<&()>, Some(&req), Some(headers))
            .await
    }

    async fn commit(&self, group: &str, consumer: &str, req: Vec<CommitRequest>) -> Result<CommitResponse> {
        let url = format!("{}commit/{}/{}", &self.url, group, consumer);
        self.client.post(&url, Option::None::<&()>, Some(&req), None).await
    }

    async fn list_consumers(&self) -> Result<Vec<GroupInstance>> {
        let url = format!("{}consumers", &self.url);
        self.client.get(&url, Option::None::<&()>).await
    }

    async fn delete_consumer(&self, group: &str, consumer: &str) -> Result<DeleteConsumerResponse> {
        let url = format!("{}delete-consumer/{}/{}", &self.url, group, consumer);
        self.client.delete(&url, Option::None::<&()>).await
    }
}
