use error::{Context, Error, ErrorKind, Result};
use once_cell::sync::OnceCell;

pub mod error;
mod kafka;

pub use kafka::*;
use serde::{de::DeserializeOwned, Serialize};

static INSTANCE: OnceCell<Client> = OnceCell::new();

pub struct Handler<'client> {
    client: &'client Client,
    url: url::Url,
}

impl<'client> Handler<'client> {
    fn new(client: &'client Client, path: &str) -> Self {
        let mut url = client.base_url.clone();
        url.set_path(path);
        Self { client, url }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    inner: reqwest::Client,
    base_url: url::Url,
}

impl Client {
    pub fn from_env() -> Self {
        let inner = reqwest::Client::new();
        let email = std::env::var("UPSTASH_EMAIL").expect("UPSTASH_EMAIL not set");
        let api_key = std::env::var("UPSTASH_API_KEY").expect("UPSTASH_API_KEY not set");
        let mut base_url = url::Url::parse("https://api.upstash.com/v2").unwrap();
        base_url.set_username(&email).unwrap();
        base_url.set_password(Some(&api_key)).unwrap();
        Self { inner, base_url }
    }

    pub fn initialize(self) {
        INSTANCE.set(self).unwrap()
    }

    pub fn instance() -> Option<&'static Self> {
        INSTANCE.get()
    }

    pub fn kafka(&self) -> Handler {
        Handler::new(self, "v2/kafka")
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

    pub async fn post<A, P, T, R>(&self, route: A, parameters: Option<&P>, json: Option<&R>) -> Result<T>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
        T: DeserializeOwned,
        R: Serialize + ?Sized,
    {
        self._post(self.absolute_url(route)?, parameters, json)
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
    ) -> Result<reqwest::Response> {
        let mut request = self.inner.post(url);
        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }
        if let Some(json) = json {
            request = request.json(json);
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
        Client::from_env().initialize();
        let client = Client::instance().unwrap();

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
        Client::from_env().initialize();
        let client = Client::instance().unwrap();

        let list = client.kafka().list_clusters().await.unwrap();
        println!("Upstash List Clusters:\n{:#?}", &list);
    }

    #[tokio::test]
    async fn get_cluster() {
        // cargo test -p upstash --lib -- tests::get_cluster --exact --nocapture

        dotenv::dotenv().unwrap();
        Client::from_env().initialize();
        let client = Client::instance().unwrap();

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
        Client::from_env().initialize();
        let client = Client::instance().unwrap();

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
        Client::from_env().initialize();
        let client = Client::instance().unwrap();

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
        Client::from_env().initialize();
        let client = Client::instance().unwrap();

        let result = client
            .kafka()
            .delete_cluster("1b729d79-0ac1-49cc-8226-ce55d5641e6a")
            .await
            .unwrap();
        println!("Upstash Delete Cluster:\n{:#?}", &result);
    }

    #[tokio::test]
    async fn create_topic() {
        dotenv::dotenv().unwrap();
        Client::from_env().initialize();
        let client = Client::instance().unwrap();

        let req = CreateTopicRequest {
            name: "kafka".to_string(),
            partitions: 1,
            retention_time: 1234,
            retention_size: 4567,
            max_message_size: 8912,
            cleanup_policy: CreateTopicCleanupPolicy::Delete,
            cluster_id: "27215fb2-bf89-4b15-a260-4ab262529957".to_string(),
        };
        client.kafka().create_topic(req).await.unwrap();
    }

    #[tokio::test]
    async fn get_topic() {
        dotenv::dotenv().unwrap();

        Client::from_env().initialize();
        let client = Client::instance().unwrap();
        client
            .kafka()
            .get_topic("f16da905-1cb2-491b-9ebd-b53e6e208c41")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn list_topic() {
        dotenv::dotenv().unwrap();

        Client::from_env().initialize();
        let client = Client::instance().unwrap();
        client
            .kafka()
            .list_topics("27215fb2-bf89-4b15-a260-4ab262529957")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn reconfigure_topic() {
        dotenv::dotenv().unwrap();

        Client::from_env().initialize();
        let client = Client::instance().unwrap();
        let req = ReconfigureTopicRequest {
            retention_time: Some(1235),
            retention_size: Some(4568),
            max_message_size: Some(8910),
        };
        client
            .kafka()
            .reconfigure_topic(req, "f16da905-1cb2-491b-9ebd-b53e6e208c41")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn delete_topic() {
        dotenv::dotenv().unwrap();

        Client::from_env().initialize();
        let client = Client::instance().unwrap();
        client
            .kafka()
            .delete_topic("c0f339d2-afe8-4c88-a503-0390cc2607b9")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn create_credential() {
        dotenv::dotenv().unwrap();

        Client::from_env().initialize();
        let client = Client::instance().unwrap();

        let req = CreateCredentialRequest {
            credential_name: "cred2".to_string(),
            topic: "tesTss".to_string(),
            permissions: CredentialPermissions::Produce,
            cluster_id: "27215fb2-bf89-4b15-a260-4ab262529957".to_string(),
        };
        let a = client.kafka().create_credential(req).await.unwrap();
        println!("create = {:?}", &a);
    }

    #[tokio::test]
    async fn list_credentials() {
        dotenv::dotenv().unwrap();
        Client::from_env().initialize();

        let client = Client::instance().unwrap();
        let a = client.kafka().list_credentials().await.unwrap();
        println!("list = {:?}", &a);
    }

    #[tokio::test]
    async fn delete_credential() {
        dotenv::dotenv().unwrap();

        Client::from_env().initialize();
        let client = Client::instance().unwrap();
        let a = client
            .kafka()
            .delete_credential("96193bc2-ffba-47d6-9485-4de5958671e3")
            .await
            .unwrap();
        println!("delete = {:?}", &a);
    }
}
