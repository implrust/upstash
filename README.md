# Upstash Kafka REST API client in Rust

## About

This Rust library is for Upstash kafka REST API client, used access kafka end points, which is documented in https://developer.upstash.com/#kafka

## Step 1: Create API Key
---
1. Create an account in upstash,<br/> **Ex: demo@mail.com**
2. Login to https://console.upstash.com
3. Create an api key from **Management Api** tab in https://console.upstash.com/account/api <br/> **Ex: a0f9f7fc-7d5c-4202-8aa8-719015fc5a7c**

## Step 2: Project Setup
---
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
---
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
---
There are lot more api endpoints available for kafka service.<br/> 
To know more about api endpoints refer: https://developer.upstash.com/#kafka <br/>
Here some rest api actions are documented.<br/>
All tests are available in lib.rs test module.

### 1. Creating Kafka Cluster
---

As per the documentation in https://developer.upstash.com/#kafka <br/> 
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
client.kafka().create_cluster(req).await.unwrap();
```
this method will create cluster from kafka service with given request data.<br>
you can test using the command: 

```rs
cargo test -p upstash --lib -- tests::create_client --exact --nocapture
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
 ClusterResponse { 
    cluster_id: "1b729d79-0ac1-49cc-8226-aaaaaaaaaaaa", 
    name: "demo", 
    region: "eu-west-1", 
    type_name: "free", 
    multizone: Some(true), 
    tcp_endpoint: "some-server-eu1-kafka.upstash.io", 
    rest_endpoint: "some-server-eu1-rest-kafka.upstash.io", 
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