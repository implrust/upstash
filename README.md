# Upstash Kafka REST API client in Rust

## About

This Rust library is for Upstash kafka REST API client, used to access kafka end points, which is documented in https://developer.upstash.com/#kafka

## Step 1: Create API Key

1. Create an account in upstash,<br/> **Ex: demo@mail.com**
2. Login to https://console.upstash.com
3. Create an api key from **Management Api** tab in https://console.upstash.com/account/api <br/> **Ex: a0f9f7fc-7d5c-4202-8aa8-719015fc5a7c**

## Step 2: Project Setup

As a rust library, you can use this in your binary rust project, here it is documented as a standalone library's cargo test, for that clone this repository and do the following,

1. create **.env** file in the root folder
2. create two variables **UPSTASH_EMAIL, UPSTASH_API_KEY** <br/>

.env
--- 
```
# For upstash operations 
UPSTASH_EMAIL='demo@email.com'
UPSTASH_API_KEY='a0f9f7fc-7d5c-4202-8aa8-719015fc5a7c'
# For kafka operations
KAFKA_REST_SERVER='glowing-crab-5802-eu1-rest-kafka.upstash.io'
KAFKA_USERNAME='Z2xvd2luZy1jcmFiLTU4MDIk97vR6sOhR7IloEyH3f_5tOF6POOj-KlEgXbF7QA'
KAFKA_PASSWORD='HVDHK7ddUTy_BT69rxHxBaNKbCH46vqW6GjOYbKjxMMDusnc1yXLRUz7_7iFfdV7'

```
if you are using this library from rust binary project means also, above two steps are required.

## Step 3: Calling / Testing - Setup

Before calling or testing rest api endpoints, it is required to initialize <br/>
    1. env file <br/>
    2. client <br/>
```rs
dotenv::dotenv().unwrap();
/// For upstash operations
Client::upstash_env().initialize();
/// For kafka operations
Client::kafka_env().initialize();
```
if you are in binary project, then place the above code in main.rs<br/>
and get the client instance where you want, either in module methods or in test cases.
```rs
/// For upstash operations
let client = Client::upstash_instance().unwrap();
/// For kafka operations
let client = Client::kafka_instance().unwrap();
```

## Step 4: Calling / Testing - REST API Endpoints

There are lot more api endpoints available for kafka service.<br/> 
To know more about api endpoints refer: https://developer.upstash.com/#kafka <br/>
Here some rest api actions are documented.<br/>
All tests are available in lib.rs test module.<br>
For more details, kindly refer source<br>

List of methods<br>
1. Creating Kafka Cluster
2. List Kafka Clusters
3. Get Kafka Cluster
4. Rename Kafka Cluster
5. Reset password for Kafka Cluster
6. Delete Kafka Cluster
7. Create Kafka Topic
8. Get Kafka Topic
9. List Kafka Topics
10. Reconfigure Kafka Topic
11. Delete Kafka Topic
12. Create Kafka Credential
13. List Kafka Credentials
14. Delete Kafka Credential
15. Get Kafka Cluster Stats
16. Get Kafka Topic Stats

The above are upstash related rest api, and still some more are there,
they related to kafka operations like producer, consumer.

List of Kafka operations<br>
1. Produce
2. Fetch
3. Consume
4. Commit
5. List Consumers
6. Delete Consumer

For these operations, you have to initialize using kafka_env(), and get client instance from kafka_instance().

### 1. Creating Kafka Cluster
---
To create cluster, endpoint url is https://api.upstash.com/v2/kafka/cluster <br/>
and it is a POST request, so, it needs request data {name, region, multizone}:
In this library parameter "region" is defined as an enum, which has two variants,

```rs
CreateClusterRegion {
    #[serde(rename = "us-east-1")]
    UsEast1,
    #[serde(rename = "eu-west-1")]
    EuWest1,
}
```
create post request data:

```rs
let req = CreateClusterRequest {
    name: "demo".to_string(),
    region: CreateClusterRegion::EuWest1,
    multizone: false,
};
```
calling create cluster method in test:
```rs
let response = client.kafka().create_cluster(req).await.unwrap();
println!("Upstash Create Cluster:\n{:#?}", &response);
```
this method will create cluster from kafka service with given request data.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::create_cluster --exact --nocapture
```
If you are calling from binary project, you have to handle error, or probagate it. 
```rs
    let result = client.kafka().create_cluster(req).await;
    match result {
        Ok(resp) => println!("response: {:?}", resp),
        Err(e) => println!("Error: {:?}", e.to_string()),
    }
```
the response object:
```rs
 Upstash Create Cluster:
 
 ClusterResponse { 
    cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a", 
    name: "implrust", 
    region: "eu-west-1", 
    type_name: "free", 
    multizone: Some(true), 
    tcp_endpoint: "glowing-crab-5802-eu1-kafka.upstash.io", 
    rest_endpoint: "glowing-crab-5802-eu1-rest-kafka.upstash.io", 
    state: "active", 
    username: "Z2xvd2luZy1jcmFiLTU4MDIk97vR6sOhR7IloEyH3f_5tOF6POOj-KlEgXbF7QA", 
    password: "keatJqqzkBugKbMU917KRk4f-KmpA1GKUNs7TZV4LF0X0Ug6pYjcZr7HfdExMmkOoFLp1w==", 
    max_retention_size: 268435456, 
    max_retention_time: 604800000, 
    max_messages_per_second: 1000, 
    creation_time: 1666182558, 
    max_message_size: 1048576, 
    max_partitions: 10 
}
```
### 2. List Kafka Clusters
---
To list all user created clusters, <br/>
Endpoint: https://api.upstash.com/v2/kafka/clusters <br/>
and it is a GET request,

calling lists clusters method in test:
```rs
let list = client.kafka().list_clusters().await.unwrap();
println!("Upstash List Clusters:\n{:#?}", &list);
```
this method will list all user created clusters from kafka service.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::list_clusters --exact --nocapture
```
response is a vector of clusters<br>
the response object:
```rs
 Upstash List Clusters:

 [
    ClusterResponse { 
        cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a", 
        name: "implrust", 
        region: "eu-west-1", 
        type_name: "free", 
        multizone: Some(true), 
        tcp_endpoint: "glowing-crab-5802-eu1-kafka.upstash.io", 
        rest_endpoint: "glowing-crab-5802-eu1-rest-kafka.upstash.io", 
        state: "active", 
        username: "Z2xvd2luZy1jcmFiLTU4MDIk97vR6sOhR7IloEyH3f_5tOF6POOj-KlEgXbF7QA", 
        password: "keatJqqzkBugKbMU917KRk4f-KmpA1GKUNs7TZV4LF0X0Ug6pYjcZr7HfdExMmkOoFLp1w==", 
        max_retention_size: 268435456, 
        max_retention_time: 604800000, 
        max_messages_per_second: 1000, 
        creation_time: 1666182558, 
        max_message_size: 1048576, 
        max_partitions: 10 
    }
 ]
```
### 3. Get Kafka Cluster
---
To Get single user created cluster, <br/>
Endpoint: https://api.upstash.com/v2/kafka/cluster/:id <br/>
and it is a GET request, here :id is a cluster id,<br>

calling get cluster method in test:
```rs
let cluster = client
    .kafka()
    .get_cluster("1b729d79-0ac1-49cc-8226-ce55d5641e6a")
    .await
    .unwrap();
println!("Upstash Get Cluster:\n{:#?}", &cluster);
```
this method will get single cluster from kafka service.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::get_cluster --exact --nocapture
```

the response object:
```rs
Upstash Get Cluster:

ClusterResponse {
    cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a",
    name: "implrust",
    region: "eu-west-1",
    type_name: "free",
    multizone: Some(true),
    tcp_endpoint: "glowing-crab-5802-eu1-kafka.upstash.io",
    rest_endpoint: "glowing-crab-5802-eu1-rest-kafka.upstash.io",
    state: "active",
    username: "Z2xvd2luZy1jcmFiLTU4MDIk97vR6sOhR7IloEyH3f_5tOF6POOj-KlEgXbF7QA",
    password: "keatJqqzkBugKbMU917KRk4f-KmpA1GKUNs7TZV4LF0X0Ug6pYjcZr7HfdExMmkOoFLp1w==",
    max_retention_size: 268435456,
    max_retention_time: 604800000,
    max_messages_per_second: 1000,
    creation_time: 1666182558,
    max_message_size: 1048576,
    max_partitions: 10,
}
```
### 4. Rename Kafka Cluster
---
To rename single cluster, <br/>
Endpoint: https://api.upstash.com/v2/kafka/rename-cluster/:id <br/>
and it is a POST request, here :id is a cluster id,<br>

Post request data:
```rs
let req = RenameClusterRequest {
    name: "implcrab".to_string(),
};
```        

calling rename cluster method in test:
```rs
let cluster = client
    .kafka()
    .rename_cluster(req, "1b729d79-0ac1-49cc-8226-ce55d5641e6a")
    .await
    .unwrap();
println!("Upstash Rename Cluster:\n{:#?}", &cluster);
```
this method will rename cluster name from kafka service with given request data.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::rename_cluster --exact --nocapture
```

the response object:
```rs
Upstash Rename Cluster:

ClusterResponse {
    cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a",
    name: "implcrab",
    region: "eu-west-1",
    type_name: "free",
    multizone: Some(true),
    tcp_endpoint: "glowing-crab-5802-eu1-kafka.upstash.io",
    rest_endpoint: "glowing-crab-5802-eu1-rest-kafka.upstash.io",
    state: "active",
    username: "Z2xvd2luZy1jcmFiLTU4MDIk97vR6sOhR7IloEyH3f_5tOF6POOj-KlEgXbF7QA",
    password: "keatJqqzkBugKbMU917KRk4f-KmpA1GKUNs7TZV4LF0X0Ug6pYjcZr7HfdExMmkOoFLp1w==",
    max_retention_size: 268435456,
    max_retention_time: 604800000,
    max_messages_per_second: 1000,
    creation_time: 1666182558,
    max_message_size: 1048576,
    max_partitions: 10,
}
```
### 5. Reset password for Kafka Cluster
---
To reset password of a single cluster, <br/>
Endpoint: https://api.upstash.com/v2/kafka/reset-password/:id <br/>
and it is a POST request, here :id is a cluster id,<br>

calling reset password cluster method in test:
```rs
let cluster = client
    .kafka()
    .reset_password("1b729d79-0ac1-49cc-8226-ce55d5641e6a")
    .await
    .unwrap();
println!("Upstash Reset Password:\n{:#?}", &cluster);
```
this method will reset cluster password from kafka service.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::reset_password --exact --nocapture
```

the response object:
```rs
Upstash Reset Password:

ClusterResponse {
    cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a",
    name: "implcrab",
    region: "eu-west-1",
    type_name: "free",
    multizone: Some(true),
    tcp_endpoint: "glowing-crab-5802-eu1-kafka.upstash.io",
    rest_endpoint: "glowing-crab-5802-eu1-rest-kafka.upstash.io",
    state: "active",
    username: "Z2xvd2luZy1jcmFiLTU4MDIk97vR6sOhR7IloEyH3f_5tOF6POOj-KlEgXbF7QA",
    password: "HVDHK7ddUTy_BT69rxHxBaNKbCH46vqW6GjOYbKjxMMDusnc1yXLRUz7_7iFfdV7",
    max_retention_size: 268435456,
    max_retention_time: 604800000,
    max_messages_per_second: 1000,
    creation_time: 1666182558,
    max_message_size: 1048576,
    max_partitions: 10,
}
```
### 6. Delete Kafka Cluster
---
To delete a single cluster, <br/>
Endpoint: https://api.upstash.com/v2/kafka/cluster/:id <br/>
and it is a DELETE request, here :id is a cluster id,<br>

calling delete cluster method in test:
```rs
let result = client
    .kafka()
    .delete_cluster("1b729d79-0ac1-49cc-8226-ce55d5641e6a")
    .await
    .unwrap();
println!("Upstash Delete Cluster:\n{:#?}", &result);
```
this method will delete cluster from kafka service.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::delete_cluster --exact --nocapture
```

the response is a string
```rs
Upstash Delete Cluster:

"OK"
```
### 7. Create Kafka Topic
---
To create topic, <br>
Endpoint: https://api.upstash.com/v2/kafka/cluster <br/>
and it is a POST request, so, it needs request data, which has a field named cleanup_policy,<br>
its is defined as an enum, which has two variants,<br>

```rs
pub enum CreateTopicCleanupPolicy {
    Compact,
    Delete,
}
```
retention_time is maintained in milliseconds<br>
1 Hr = (1 min * 60) = (60 sec * 60) = (1000 ms * 60 * 60) = 3600000 ms<br>
24 Hr = 86400000 ms<br>
retention_size is maintained in bits<br>
1 MB = 1024 kb = (1024 bits * 1024 bits) = 1048576 bits<br>
256 MB = 268435456 bits<br>
max_message is maintained in bytes<br>
100 KB = (1 kb * 100) = 102400 bytes<br>
500 KB = 512000 Bytes<br>

create post request data:

```rs
let req = CreateTopicRequest {
    name: "one".to_string(),
    partitions: 1,
    retention_time: 3600000,
    retention_size: 1048576,
    max_message_size: 102400,
    cleanup_policy: CreateTopicCleanupPolicy::Delete,
    cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a".to_string(),
};
```
calling create topic method in test:
```rs
let topic = client.kafka().create_topic(req).await.unwrap();
println!("Upstash Create Topic:\n{:#?}", &topic);
```
this method will create topic from kafka service with given request data.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::create_topic --exact --nocapture
```
the response object:
```rs
Upstash Create Topic:
TopicResponse {
    topic_id: "30f59d3d-a561-46e3-9f5d-d5e55a4519b2",
    topic_name: "one",
    cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a",
    region: "eu-west-1",
    creation_time: 1666369909,
    state: "pending",
    partitions: 1,
    multizone: Some(true),
    tcp_endpoint: "glowing-crab-5802-eu1-kafka.upstash.io",
    rest_endpoint: "glowing-crab-5802-eu1-rest-kafka.upstash.io",
    username: "Z2xvd2luZy1jcmFiLTU4MDIk97vR6sOhR7IloEyH3f_5tOF6POOj-KlEgXbF7QA",
    password: "HVDHK7ddUTy_BT69rxHxBaNKbCH46vqW6GjOYbKjxMMDusnc1yXLRUz7_7iFfdV7",
    cleanup_policy: "delete",
    retention_size: 1048576,
    retention_time: 3600000,
    max_message_size: 102400,
}
```
### 8. Get Kafka Topic
---
To get kafka topic,<br/>
Endpoint: https://api.upstash.com/v2/kafka/topic/:id <br/>
and it is a GET request, here :id is a topic id,<br>

calling get topic method in test:
```rs
let topic = client
    .kafka()
    .get_topic("30f59d3d-a561-46e3-9f5d-d5e55a4519b2")
    .await
    .unwrap();
println!("Upstash Kafka Topic:\n{:#?}", &topic);
```
this method will get requested topic from kafka service.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::get_topic --exact --nocapture
```

the response object:
```rs
Upstash Kafka Topic:

TopicResponse {
    topic_id: "30f59d3d-a561-46e3-9f5d-d5e55a4519b2",
    topic_name: "one",
    cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a",
    region: "eu-west-1",
    creation_time: 1666369909,
    state: "active",
    partitions: 1,
    multizone: Some(true),
    tcp_endpoint: "glowing-crab-5802-eu1-kafka.upstash.io",
    rest_endpoint: "glowing-crab-5802-eu1-rest-kafka.upstash.io",
    username: "Z2xvd2luZy1jcmFiLTU4MDIk97vR6sOhR7IloEyH3f_5tOF6POOj-KlEgXbF7QA",
    password: "HVDHK7ddUTy_BT69rxHxBaNKbCH46vqW6GjOYbKjxMMDusnc1yXLRUz7_7iFfdV7",
    cleanup_policy: "delete",
    retention_size: 1048576,
    retention_time: 3600000,
    max_message_size: 102400,
}
```
### 9. List Kafka Topics
---
To List kafka topics,<br/>
Endpoint: https://api.upstash.com/v2/kafka/topics/:id <br/>
and it is a GET request, here :id is a cluster id,<br>

calling get topic list method in test:
```rs
let topics = client
    .kafka()
    .list_topics("1b729d79-0ac1-49cc-8226-ce55d5641e6a")
    .await
    .unwrap();
println!("Upstash Kafka Topic List:\n{:#?}", &topics);
```
this method will get all topics for given cluster id from kafka service.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::list_topics --exact --nocapture
```

the response object:
```rs
Upstash Kafka Topic List:
[
    TopicResponse {
        topic_id: "c5d3abad-f1d6-4726-88e9-c96ec88b38a7",
        topic_name: "rust",
        cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a",
        region: "eu-west-1",
        creation_time: 1666370187,
        state: "active",
        partitions: 1,
        multizone: Some(true),
        tcp_endpoint: "glowing-crab-5802-eu1-kafka.upstash.io",
        rest_endpoint: "glowing-crab-5802-eu1-rest-kafka.upstash.io",
        username: "Z2xvd2luZy1jcmFiLTU4MDIk97vR6sOhR7IloEyH3f_5tOF6POOj-KlEgXbF7QA",
        password: "HVDHK7ddUTy_BT69rxHxBaNKbCH46vqW6GjOYbKjxMMDusnc1yXLRUz7_7iFfdV7",
        cleanup_policy: "compact",
        retention_size: -1,
        retention_time: 604800000,
        max_message_size: 102400,
    },
    TopicResponse {
        topic_id: "30f59d3d-a561-46e3-9f5d-d5e55a4519b2",
        topic_name: "one",
        cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a",
        region: "eu-west-1",
        creation_time: 1666369909,
        state: "active",
        partitions: 1,
        multizone: Some(true),
        tcp_endpoint: "glowing-crab-5802-eu1-kafka.upstash.io",
        rest_endpoint: "glowing-crab-5802-eu1-rest-kafka.upstash.io",
        username: "Z2xvd2luZy1jcmFiLTU4MDIk97vR6sOhR7IloEyH3f_5tOF6POOj-KlEgXbF7QA",
        password: "HVDHK7ddUTy_BT69rxHxBaNKbCH46vqW6GjOYbKjxMMDusnc1yXLRUz7_7iFfdV7",
        cleanup_policy: "delete",
        retention_size: 1048576,
        retention_time: 3600000,
        max_message_size: 102400,
    },
    TopicResponse {
        topic_id: "ea3dfdd5-671a-4b46-aed9-46a8276a39b3",
        topic_name: "crab",
        cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a",
        region: "eu-west-1",
        creation_time: 1666370095,
        state: "active",
        partitions: 1,
        multizone: Some(true),
        tcp_endpoint: "glowing-crab-5802-eu1-kafka.upstash.io",
        rest_endpoint: "glowing-crab-5802-eu1-rest-kafka.upstash.io",
        username: "Z2xvd2luZy1jcmFiLTU4MDIk97vR6sOhR7IloEyH3f_5tOF6POOj-KlEgXbF7QA",
        password: "HVDHK7ddUTy_BT69rxHxBaNKbCH46vqW6GjOYbKjxMMDusnc1yXLRUz7_7iFfdV7",
        cleanup_policy: "delete",
        retention_size: 1048576,
        retention_time: 3600000,
        max_message_size: 102400,
    },
]
```
### 10. Reconfigure Kafka Topic
---
To reconfigure topic,<br>
Endpoint: https://api.upstash.com/v2/kafka/update-topic/:id<br/>
and it is a POST request, here :id is a topic id<br>
POST request needs data {retention_time, retention_size, max_message_size}<br>
you can reconfigure any one or all parameters, so 3 parameters are optional<br>

create post request data:

```rs
let req = ReconfigureTopicRequest {
    retention_time: Some(86400000),
    retention_size: Some(268435456),
    max_message_size: Some(512000),
};
```
calling reconfigure topic method in test:
```rs
let result = client
    .kafka()
    .reconfigure_topic(req, "ea3dfdd5-671a-4b46-aed9-46a8276a39b3")
    .await
    .unwrap();
println!("Upstash Kafka Reconfigure Topic:\n{:#?}", &result);
```
this method will reconfigure topic from kafka service with given request data.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::reconfigure_topic --exact --nocapture
```
the response object:
```rs
Upstash Kafka Reconfigure Topic:

TopicResponse {
    topic_id: "ea3dfdd5-671a-4b46-aed9-46a8276a39b3",
    topic_name: "crab",
    cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a",
    region: "eu-west-1",
    creation_time: 1666370095,
    state: "active",
    partitions: 1,
    multizone: Some(true),
    tcp_endpoint: "glowing-crab-5802-eu1-kafka.upstash.io",
    rest_endpoint: "glowing-crab-5802-eu1-rest-kafka.upstash.io",
    username: "Z2xvd2luZy1jcmFiLTU4MDIk97vR6sOhR7IloEyH3f_5tOF6POOj-KlEgXbF7QA",
    password: "HVDHK7ddUTy_BT69rxHxBaNKbCH46vqW6GjOYbKjxMMDusnc1yXLRUz7_7iFfdV7",
    cleanup_policy: "delete",
    retention_size: 268435456,
    retention_time: 86400000,
    max_message_size: 512000,
}
```
### 11. Delete Kafka Topic
---
To delete a kafka topic, <br/>
Endpoint: https://api.upstash.com/v2/kafka/topic/:id <br/>
and it is a DELETE request, here :id is a topic id,<br>

calling delete topic method in test:
```rs
let result = client
    .kafka()
    .delete_topic("ea3dfdd5-671a-4b46-aed9-46a8276a39b3")
    .await
    .unwrap();
println!("Upstash Delete Topic:\n{:#?}", &result);
```
this method will delete topic from kafka service.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::delete_topic --exact --nocapture
```

the response is a string
```rs
Upstash Delete Topic:

"OK"
```
### 12. Create Kafka Credential
---
To create kafka credential for specific topic or all(*) with permission PRODUCE/CONSUME/ALL,<br>
Endpoint: https://api.upstash.com/v2/kafka/credential<br/>
and it is a POST request, so, it needs request data { credential_name, cluster_id, topic, permissions }<br>

In this method "permission" is defined as an enum, which has three variants,
```rs
pub enum CredentialPermissions {
    All,
    Produce,
    Consume,
}
```
create post request data:

```rs
let req = CreateCredentialRequest {
    credential_name: "generate".to_string(),
    topic: "one".to_string(),
    permissions: CredentialPermissions::Produce,
    cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a".to_string(),
};
// or
let req = CreateCredentialRequest {
    credential_name: "full".to_string(),
    topic: "*".to_string(),
    permissions: CredentialPermissions::All,
    cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a".to_string(),
};
```
calling create credential method in test:
```rs
let credential = client.kafka().create_credential(req).await.unwrap();
println!("Upstash Kafka Credential:{:?}", &credential);
```
this method will create credential from kafka service with given request data.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::create_credential --exact --nocapture
```
the response object:
```rs
Upstash Kafka Credential:

CredentialResponse {
    credential_id: "b6022d46-6279-4b4a-88a1-f8d9d74263f5",
    credential_name: "Generate",
    topic: "one",
    permissions: Produce,
    cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a",
    username: "glowing-crab-5802",
    creation_time: 1666509417,
    state: Active,
    password: "ncDZT2jBuSfwi2nu8L3zM2Eh82OznpfDBLyuoWtzkFWsRNS0riqti--UE5RRzu8H8-Lm8N7DK2Pp57HmExV0xV15VHP5T_bbiKOCLjqAKds=",
    encoded_username: "Z2xvd2luZy1jcmFiLTU4MDIkA1mzprnymGV4a3INrXGKmf2ldAwdvhACrQu1Xdc",
}
```
### 13. List Kafka Credentials
---
To List kafka credentials,<br/>
Endpoint: https://api.upstash.com/v2/kafka/credentials<br/>
and it is a GET request<br>
There is something to note here, the default credential with full access for all topics is created inbuilt.<br>
it won't be listed. the list contains only what you have created.

calling list credentials method in test:
```rs
let credentials = client.kafka().list_credentials().await.unwrap();
println!("Upstash Kafka Credentials:\n{:?}", &credentials);
```
this method will list all credentials of your cluster from kafka service.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::list_credentials --exact --nocapture
```

the response object:
```rs
Upstash Kafka Credentials:

[
    CredentialResponse {
        credential_id: "b6022d46-6279-4b4a-88a1-f8d9d74263f5",
        credential_name: "Generate",
        topic: "one",
        permissions: Produce,
        cluster_id: "1b729d79-0ac1-49cc-8226-ce55d5641e6a",
        username: "glowing-crab-5802",
        creation_time: 1666509417,
        state: Active,
        password: "ncDZT2jBuSfwi2nu8L3zM2Eh82OznpfDBLyuoWtzkFWsRNS0riqti--UE5RRzu8H8-Lm8N7DK2Pp57HmExV0xV15VHP5T_bbiKOCLjqAKds=",
        encoded_username: "Z2xvd2luZy1jcmFiLTU4MDIkA1mzprnymGV4a3INrXGKmf2ldAwdvhACrQu1Xdc",
    },
]
```
### 14. Delete Kafka Credential
---
To delete a kafka credential, <br/>
Endpoint: https://api.upstash.com/v2/kafka/credential/:id <br/>
and it is a DELETE request, here :id is a credential id,<br>

calling delete credential method in test:
```rs
let result = client
    .kafka()
    .delete_credential("b6022d46-6279-4b4a-88a1-f8d9d74263f5")
    .await
    .unwrap();
println!("Upstash Delete Credential:\n{:#?}", &result);
```
this method will delete credential from kafka service.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::delete_credential --exact --nocapture
```

the response is a string
```rs
Upstash Delete Credential:

"OK"
```
### 15. Get Kafka Cluster Stats
---
To get kafka cluster statistics,<br>
Endpoint: https://api.upstash.com/v2/kafka/stats/cluster/:id<br>
its a GET request, here :id is a cluster id<br>

calling get kafka stats method in test:
```rs
let result = client
    .kafka()
    .cluster_stats("1b729d79-0ac1-49cc-8226-ce55d5641e6a")
    .await
    .unwrap();
println!("Upstash Cluster Stats:\n{:#?}", &result);
```
this method will get kafka cluster stistics from kafka service.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::cluster_stats --exact --nocapture
```
the response is a ClusterStats
```rs
Upstash Cluster Stats:

ClusterStats {
    throughput: [
        Stat {
            x: "2022-10-23 07:06:22",
            y: 0,
        },
        ...
    ],
    produce_throughput: [
        Stat {
            x: "2022-10-23 07:06:22",
            y: 0,
        },
        ...
    ],
    consume_throughput: [
        Stat {
            x: "2022-10-23 07:06:22",
            y: 0,
        },
        ...
    ],
    diskusage: [
        Stat {
            x: "2022-10-22 08:35:22",
            y: 0,
        },
        ...
    ],
    days: [
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ],
    dailyproduce: [
        Stat {
            x: "2022-10-19 08:21:12.381310405 +0000 UTC",
            y: 0,
        },
        ...
    ],
    dailyconsume: [
        Stat {
            x: "2022-10-19 08:21:12.381308648 +0000 UTC",
            y: 0,
        },
        ...
    ],
    total_monthly_storage: 0,
    total_monthly_billing: 0,
    total_monthly_produce: 0,
    total_monthly_consume: 0,
}
```

### 16. Get Kafka Topic Stats
---
To get kafka topic statistics,<br>
Endpoint: https://api.upstash.com/v2/kafka/stats/topic/:id<br>
its a GET request, here :id is a topic id<br>

calling get kafka stats method in test:
```rs
let result = client
    .kafka()
    .topic_stats("30f59d3d-a561-46e3-9f5d-d5e55a4519b2")
    .await
    .unwrap();
println!("Upstash Topic Stats:\n{:#?}", &result);
```
this method will get kafka topic stistics from kafka service.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::topic_stats --exact --nocapture
```
the response is a TopicStats
```rs
Upstash Topic Stats:

TopicStats {
    throughput: [
        Stat {
            x: "2022-10-23 07:06:22",
            y: 0,
        },
        ...
    ],
    produce_throughput: [
        Stat {
            x: "2022-10-23 07:06:22",
            y: 0,
        },
        ...
    ],
    consume_throughput: [
        Stat {
            x: "2022-10-23 07:06:22",
            y: 0,
        },
        ...
    ],
    diskusage: [
        Stat {
            x: "2022-10-22 08:35:22",
            y: 0,
        },
        ...
    ],
    total_monthly_storage: 0,
    total_monthly_produce: 0,
    total_monthly_consume: 0,
}
```
