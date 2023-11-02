// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/otterize/helm-charts/k8s.otterize.com/v1alpha3/clientintents.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// IntentsSpec defines the desired state of ClientIntents
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "k8s.otterize.com", version = "v1alpha3", kind = "ClientIntents", plural = "clientintents")]
#[kube(namespaced)]
#[kube(status = "ClientIntentsStatus")]
#[kube(schema = "disabled")]
pub struct ClientIntentsSpec {
    pub calls: Vec<ClientIntentsCalls>,
    pub service: ClientIntentsService,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientIntentsCalls {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "HTTPResources")]
    pub http_resources: Option<Vec<ClientIntentsCallsHttpResources>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "databaseResources")]
    pub database_resources: Option<Vec<ClientIntentsCallsDatabaseResources>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kafkaTopics")]
    pub kafka_topics: Option<Vec<ClientIntentsCallsKafkaTopics>>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<ClientIntentsCallsType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientIntentsCallsHttpResources {
    pub methods: Vec<String>,
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientIntentsCallsDatabaseResources {
    pub operations: Vec<String>,
    pub table: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientIntentsCallsKafkaTopics {
    pub name: String,
    pub operations: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientIntentsCallsType {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "kafka")]
    Kafka,
    #[serde(rename = "database")]
    Database,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientIntentsService {
    pub name: String,
}

/// IntentsStatus defines the observed state of ClientIntents
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientIntentsStatus {
}

