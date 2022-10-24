use error::{Context, Error, ErrorKind, Result};
use reqwest::header::{HeaderMap, HeaderValue};

use once_cell::sync::OnceCell;

pub mod error;
mod kafka;

pub use kafka::*;
use serde::{de::DeserializeOwned, Serialize};

static UPSTASH_INSTANCE: OnceCell<Client> = OnceCell::new();
static KAFKA_INSTANCE: OnceCell<Client> = OnceCell::new();

pub struct Handler<'client> {
    client: &'client Client,
    url: url::Url,
}

impl<'client> Handler<'client> {
    fn new(client: &'client Client, path: &str) -> Self {
        let mut url = client.base_url.clone();
        if !path.is_empty() {
            url.set_path(path);
        }
        Self { client, url }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    inner: reqwest::Client,
    base_url: url::Url,
}

impl Client {
    pub fn upstash_env() -> Self {
        let inner = reqwest::Client::new();
        let email = std::env::var("UPSTASH_EMAIL").expect("UPSTASH_EMAIL not set");
        let api_key = std::env::var("UPSTASH_API_KEY").expect("UPSTASH_API_KEY not set");
        let mut base_url = url::Url::parse("https://api.upstash.com/v2").unwrap();
        base_url.set_username(&email).unwrap();
        base_url.set_password(Some(&api_key)).unwrap();
        Self { inner, base_url }
    }

    pub fn kafka_env() -> Self {
        let inner = reqwest::Client::new();
        let username = std::env::var("KAFKA_USERNAME").expect("KAFKA_USERNAME not set");
        let password = std::env::var("KAFKA_PASSWORD").expect("KAFKA_PASSWORD not set");
        let rest_server = std::env::var("KAFKA_REST_SERVER").expect("KAFKA_REST_SERVER not set");
        let rest_server = format!("https://{}", &rest_server);
        let mut base_url = url::Url::parse(&rest_server).unwrap();
        base_url.set_username(&username).unwrap();
        base_url.set_password(Some(&password)).unwrap();
        Self { inner, base_url }
    }

    pub fn initialize(self) {
        if let Some(domain) = self.base_url.domain() {
            if domain == "api.upstash.com" {
                UPSTASH_INSTANCE.set(self).unwrap()
            } else {
                KAFKA_INSTANCE.set(self).unwrap()
            }
        } else {
            panic!("initialize error");
        }
    }

    pub fn upstash_instance() -> Option<&'static Self> {
        UPSTASH_INSTANCE.get()
    }

    pub fn kafka_instance() -> Option<&'static Self> {
        KAFKA_INSTANCE.get()
    }

    pub fn kafka(&self) -> Handler {
        Handler::new(self, "v2/kafka")
    }

    pub fn producer(&self) -> Handler {
        Handler::new(self, "produce")
    }
    pub fn fetcher(&self) -> Handler {
        Handler::new(self, "fetch")
    }
    pub fn consumer(&self) -> Handler {
        Handler::new(self, "consume")
    }
    pub fn handler(&self, path: &str) -> Handler {
        Handler::new(self, path)
    }
    pub fn absolute_url(&self, url: impl AsRef<str>) -> Result<url::Url> {
        self.base_url
            .join(url.as_ref())
            .map_err(|err| Error::new(err.to_string(), ErrorKind::Internal))
    }
}

impl Client {
    pub async fn get<A, P, T>(&self, route: A, parameters: Option<&P>) -> Result<T>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self._get(self.absolute_url(route)?, parameters)
            .await?
            .json()
            .await
            .map_err(|err| Error::new(err.to_string(), ErrorKind::ApiError(err.to_string())))
    }

    pub async fn delete<A, P, T>(&self, route: A, parameters: Option<&P>) -> Result<T>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self._delete(self.absolute_url(route)?, parameters)
            .await?
            .json()
            .await
            .map_err(|err| Error::new(err.to_string(), ErrorKind::ApiError(err.to_string())))
    }

    pub async fn post<A, P, T, R>(
        &self,
        route: A,
        parameters: Option<&P>,
        json: Option<&R>,
        headers: Option<HeaderMap>,
    ) -> Result<T>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
        T: DeserializeOwned,
        R: Serialize + ?Sized,
    {
        self._post(self.absolute_url(route)?, parameters, json, headers)
            .await?
            .json()
            .await
            .map_err(|err| Error::new(err.to_string(), ErrorKind::ApiError(err.to_string())))
    }

    pub async fn _get<P: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.inner.get(url);
        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }
        self.execute(request).await
    }

    pub async fn _delete<P: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.inner.delete(url);
        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }
        self.execute(request).await
    }

    pub async fn _post<P: Serialize + ?Sized, T: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        parameters: Option<&P>,
        json: Option<&T>,
        headers: Option<HeaderMap>,
    ) -> Result<reqwest::Response> {
        let mut request = self.inner.post(url);
        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }
        if let Some(json) = json {
            request = request.json(json);
        }
        if let Some(headers) = headers {
            request = request.headers(headers)
        }
        self.execute(request).await
    }

    pub async fn execute(&self, request: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        request.send().await.context("Http execution failure")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn create_cluster() {
        // cargo test -p upstash --lib -- tests::create_cluster --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::upstash_env().initialize();
        let client = Client::upstash_instance().unwrap();

        let req = CreateClusterRequest {
            name: "implrust".to_string(),
            region: CreateClusterRegion::EuWest1,
            multizone: true,
        };

        let response = client.kafka().create_cluster(req).await.unwrap();
        println!("Upstash Create Cluster:\n{:#?}", &response);
    }

    #[tokio::test]
    async fn list_clusters() {
        // cargo test -p upstash --lib -- tests::list_clusters --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::upstash_env().initialize();
        let client = Client::upstash_instance().unwrap();

        let list = client.kafka().list_clusters().await.unwrap();
        println!("Upstash List Clusters:\n{:#?}", &list);
    }

    #[tokio::test]
    async fn get_cluster() {
        // cargo test -p upstash --lib -- tests::get_cluster --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::upstash_env().initialize();
        let client = Client::upstash_instance().unwrap();

        let cluster = client
            .kafka()
            .get_cluster("1b729d79-0ac1-49cc-8226-ce55d5641e6a")
            .await
            .unwrap();
        println!("Upstash Get Cluster:\n{:#?}", &cluster);
    }

    #[tokio::test]
    async fn rename_cluster() {
        // cargo test -p upstash --lib -- tests::rename_cluster --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::upstash_env().initialize();
        let client = Client::upstash_instance().unwrap();

        let req = RenameClusterRequest {
            name: "implcrab".to_string(),
        };
        let cluster = client
            .kafka()
            .rename_cluster(req, "1b729d79-0ac1-49cc-8226-ce55d5641e6a")
            .await
            .unwrap();
        println!("Upstash Rename Cluster:\n{:#?}", &cluster);
    }

    #[tokio::test]
    async fn reset_password() {
        // cargo test -p upstash --lib -- tests::reset_password --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::upstash_env().initialize();
        let client = Client::upstash_instance().unwrap();

        let cluster = client
            .kafka()
            .reset_password("1b729d79-0ac1-49cc-8226-ce55d5641e6a")
            .await
            .unwrap();
        println!("Upstash Reset Password:\n{:#?}", &cluster);
    }

    #[tokio::test]
    async fn delete_cluster() {
        // cargo test -p upstash --lib -- tests::delete_cluster --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::upstash_env().initialize();
        let client = Client::upstash_instance().unwrap();

        let result = client
            .kafka()
            .delete_cluster("1b729d79-0ac1-49cc-8226-ce55d5641e6a")
            .await
            .unwrap();
        println!("Upstash Delete Cluster:\n{:#?}", &result);
    }

    #[tokio::test]
    async fn create_topic() {
        // cargo test -p upstash --lib -- tests::create_topic --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::upstash_env().initialize();
        let client = Client::upstash_instance().unwrap();

        let req = CreateTopicRequest {
            name: "one".to_string(),
            partitions: 1,
            retention_time: 3600000,
            retention_size: 1048576,
            max_message_size: 102400,
            cleanup_policy: CreateTopicCleanupPolicy::Compact,
            cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a".to_string(),
        };
        let topic = client.kafka().create_topic(req).await.unwrap();
        println!("Upstash Create Topic:\n{:#?}", &topic);
    }

    #[tokio::test]
    async fn get_topic() {
        // cargo test -p upstash --lib -- tests::get_topic --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::upstash_env().initialize();
        let client = Client::upstash_instance().unwrap();

        let topic = client
            .kafka()
            .get_topic("30f59d3d-a561-46e3-9f5d-d5e55a4519b2")
            .await
            .unwrap();
        println!("Upstash Kafka Topic:\n{:#?}", &topic);
    }

    #[tokio::test]
    async fn list_topics() {
        // cargo test -p upstash --lib -- tests::list_topics --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::upstash_env().initialize();
        let client = Client::upstash_instance().unwrap();

        let topics = client
            .kafka()
            .list_topics("1b729d79-0ac1-49cc-8226-ce55d5641e6a")
            .await
            .unwrap();
        println!("Upstash Kafka Topic List:\n{:#?}", &topics);
    }

    #[tokio::test]
    async fn reconfigure_topic() {
        // cargo test -p upstash --lib -- tests::reconfigure_topic --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::upstash_env().initialize();
        let client = Client::upstash_instance().unwrap();

        let req = ReconfigureTopicRequest {
            retention_time: Some(86400000),
            retention_size: Some(268435456),
            max_message_size: Some(512000),
        };
        let result = client
            .kafka()
            .reconfigure_topic(req, "ea3dfdd5-671a-4b46-aed9-46a8276a39b3")
            .await
            .unwrap();
        println!("Upstash Kafka reconfigure topic:\n{:#?}", &result);
    }

    #[tokio::test]
    async fn delete_topic() {
        // cargo test -p upstash --lib -- tests::delete_topic --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::upstash_env().initialize();
        let client = Client::upstash_instance().unwrap();

        let result = client
            .kafka()
            .delete_topic("ea3dfdd5-671a-4b46-aed9-46a8276a39b3")
            .await
            .unwrap();
        println!("Upstash Delete Topic:\n{:#?}", &result);
    }

    #[tokio::test]
    async fn create_credential() {
        // cargo test -p upstash --lib -- tests::create_credential --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::upstash_env().initialize();
        let client = Client::upstash_instance().unwrap();

        let req = CreateCredentialRequest {
            credential_name: "Generate".to_string(),
            topic: "one".to_string(),
            permissions: CredentialPermissions::Produce,
            cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a".to_string(),
        };

        let credential = client.kafka().create_credential(req).await.unwrap();
        println!("Upstash Kafka Credential:\n{:#?}", &credential);
    }

    #[tokio::test]
    async fn list_credentials() {
        // cargo test -p upstash --lib -- tests::list_credentials --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::upstash_env().initialize();
        let client = Client::upstash_instance().unwrap();

        let credentials = client.kafka().list_credentials().await.unwrap();
        println!("Upstash Kafka Credentials:\n{:#?}", &credentials);
    }

    #[tokio::test]
    async fn delete_credential() {
        // cargo test -p upstash --lib -- tests::delete_credential --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::upstash_env().initialize();
        let client = Client::upstash_instance().unwrap();

        let result = client
            .kafka()
            .delete_credential("b6022d46-6279-4b4a-88a1-f8d9d74263f5")
            .await
            .unwrap();
        println!("Upstash Delete Credential:\n{:#?}", &result);
    }

    #[tokio::test]
    async fn cluster_stats() {
        // cargo test -p upstash --lib -- tests::cluster_stats --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::upstash_env().initialize();
        let client = Client::upstash_instance().unwrap();

        let result = client
            .kafka()
            .cluster_stats("1b729d79-0ac1-49cc-8226-ce55d5641e6a")
            .await
            .unwrap();
        println!("Upstash Cluster Stats:\n{:#?}", &result);
    }

    #[tokio::test]
    async fn topic_stats() {
        // cargo test -p upstash --lib -- tests::topic_stats --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::upstash_env().initialize();
        let client = Client::upstash_instance().unwrap();

        let result = client
            .kafka()
            .topic_stats("30f59d3d-a561-46e3-9f5d-d5e55a4519b2")
            .await
            .unwrap();
        println!("Upstash Topic Stats:\n{:#?}", &result);
    }

    #[tokio::test]
    async fn produce() {
        // cargo test -p upstash --lib -- tests::produce --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::kafka_env().initialize();
        let client = Client::kafka_instance().unwrap();

        let messages = vec![
            Message::new("one", "21", Some(0), Some("key")),
            Message::new("one", "22", Some(0), Some("key")),
            Message::new("one", "23", Some(0), Some("key")),
        ];

        let response = client.producer().produce(messages).await.unwrap();
        println!("Upstash Kafka Produce:\n{:#?}", &response);
    }

    #[tokio::test]
    async fn fetch() {
        // cargo test -p upstash --lib -- tests::fetch --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::kafka_env().initialize();
        let client = Client::kafka_instance().unwrap();

        let req = FetchRequest {
            topic: "one".to_string(),
            partition: 0,
            offset: 5,
        };

        let response = client.fetcher().fetch(req).await;
        println!("Upstash Kafka Fetch:\n{:#?}", &response);
    }

    #[tokio::test]
    async fn consume() {
        // cargo test -p upstash --lib -- tests::consume --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::kafka_env().initialize();
        let client = Client::kafka_instance().unwrap();

        let req = ConsumeRequest {
            topic: "one".to_string(),
        };
        let group = "g1";
        let consumer = "c1";

        let response = client.consumer().consume(group, consumer, req).await;
        println!("Upstash Kafka Consume:\n{:#?}", &response);
    }

    #[tokio::test]
    async fn commit() {
        // cargo test -p upstash --lib -- tests::consume --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::kafka_env().initialize();
        let client = Client::kafka_instance().unwrap();

        let req = vec![
            CommitRequest::new("one", 0, 85),
            CommitRequest::new("one", 0, 86),
            CommitRequest::new("one", 0, 87),
        ];
        let group = "g1";
        let consumer = "c1";

        let response = client.handler("").commit(group, consumer, req).await;
        println!("Upstash Kafka Commit:\n{:#?}", &response);
    }

    #[tokio::test]
    async fn list_consumer() {
        // cargo test -p upstash --lib -- tests::list_consumer --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::kafka_env().initialize();
        let client = Client::kafka_instance().unwrap();

        let response = client.handler("").list_consumer().await;
        println!("Upstash Kafka List Consumer:\n{:#?}", &response);
    }

    #[tokio::test]
    async fn delete_consumer() {
        // cargo test -p upstash --lib -- tests::delete_consumer --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::kafka_env().initialize();
        let client = Client::kafka_instance().unwrap();

        let group = "g1";
        let consumer = "c1";

        let response = client.handler("").delete_consumer(group, consumer).await;
        println!("Upstash Kafka Delete Consumer:\n{:#?}", &response);
    }
}
