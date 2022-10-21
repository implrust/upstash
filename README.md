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
UPSTASH_EMAIL='demo@email.com'
UPSTASH_API_KEY='a0f9f7fc-7d5c-4202-8aa8-719015fc5a7c'
```
if you are using this library from rust binary project means also, above two steps are required.

## Step 3: Calling / Testing - Setup

Before calling or testing rest api endpoints, it is required to initialize <br/>
    1. env file <br/>
    2. client <br/>
```rs
dotenv::dotenv().unwrap();
Client::from_env().initialize();
```
if you are in binary project, then place the above code in main.rs<br/>
and get the client instance where you want, either in module methods or in test cases.
```rs
let client = Client::instance().unwrap();
```

## Step 4: Calling / Testing - REST API Endpoints

There are lot more api endpoints available for kafka service.<br/> 
To know more about api endpoints refer: https://developer.upstash.com/#kafka <br/>
Here some rest api actions are documented.<br/>
All tests are available in lib.rs test module.

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