// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/kafka-controller/kafka.services.k8s.aws/v1alpha1/clusters.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ClusterSpec defines the desired state of Cluster.
/// 
/// 
/// Returns information about a cluster of either the provisioned or the serverless
/// type.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kafka.services.k8s.aws", version = "v1alpha1", kind = "Cluster", plural = "clusters")]
#[kube(namespaced)]
#[kube(status = "ClusterStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "associatedSCRAMSecretRefs")]
    pub associated_scram_secret_refs: Option<Vec<ClusterAssociatedScramSecretRefs>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "associatedSCRAMSecrets")]
    pub associated_scram_secrets: Option<Vec<String>>,
    /// Information about the brokers.
    #[serde(rename = "brokerNodeGroupInfo")]
    pub broker_node_group_info: ClusterBrokerNodeGroupInfo,
    /// Includes all client authentication related information.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientAuthentication")]
    pub client_authentication: Option<ClusterClientAuthentication>,
    /// Represents the configuration that you want MSK to use for the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configurationInfo")]
    pub configuration_info: Option<ClusterConfigurationInfo>,
    /// Includes all encryption-related information.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptionInfo")]
    pub encryption_info: Option<ClusterEncryptionInfo>,
    /// Specifies the level of monitoring for the MSK cluster. The possible values
    /// are DEFAULT, PER_BROKER, PER_TOPIC_PER_BROKER, and PER_TOPIC_PER_PARTITION.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enhancedMonitoring")]
    pub enhanced_monitoring: Option<String>,
    /// The version of Apache Kafka.
    #[serde(rename = "kafkaVersion")]
    pub kafka_version: String,
    /// LoggingInfo details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loggingInfo")]
    pub logging_info: Option<ClusterLoggingInfo>,
    /// The name of the cluster.
    pub name: String,
    /// The number of Apache Kafka broker nodes in the Amazon MSK cluster.
    #[serde(rename = "numberOfBrokerNodes")]
    pub number_of_broker_nodes: i64,
    /// The settings for open monitoring.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openMonitoring")]
    pub open_monitoring: Option<ClusterOpenMonitoring>,
    /// This controls storage mode for supported storage tiers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageMode")]
    pub storage_mode: Option<String>,
    /// Create tags when creating the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterAssociatedScramSecretRefs {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<ClusterAssociatedScramSecretRefsFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterAssociatedScramSecretRefsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Information about the brokers.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBrokerNodeGroupInfo {
    /// The distribution of broker nodes across Availability Zones. By default, broker
    /// nodes are distributed among the Availability Zones of your Region. Currently,
    /// the only supported value is DEFAULT. You can either specify this value explicitly
    /// or leave it out.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "brokerAZDistribution")]
    pub broker_az_distribution: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientSubnets")]
    pub client_subnets: Option<Vec<String>>,
    /// Information about the broker access configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectivityInfo")]
    pub connectivity_info: Option<ClusterBrokerNodeGroupInfoConnectivityInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceType")]
    pub instance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroups")]
    pub security_groups: Option<Vec<String>>,
    /// Contains information about storage volumes attached to MSK broker nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageInfo")]
    pub storage_info: Option<ClusterBrokerNodeGroupInfoStorageInfo>,
}

/// Information about the broker access configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBrokerNodeGroupInfoConnectivityInfo {
    /// Broker public access control.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "publicAccess")]
    pub public_access: Option<ClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess>,
}

/// Broker public access control.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type_")]
    pub r#type: Option<String>,
}

/// Contains information about storage volumes attached to MSK broker nodes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBrokerNodeGroupInfoStorageInfo {
    /// Contains information about the EBS storage volumes attached to Apache Kafka
    /// broker nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ebsStorageInfo")]
    pub ebs_storage_info: Option<ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo>,
}

/// Contains information about the EBS storage volumes attached to Apache Kafka
/// broker nodes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo {
    /// Contains information about provisioned throughput for EBS storage volumes
    /// attached to kafka broker nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "provisionedThroughput")]
    pub provisioned_throughput: Option<ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeSize")]
    pub volume_size: Option<i64>,
}

/// Contains information about provisioned throughput for EBS storage volumes
/// attached to kafka broker nodes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeThroughput")]
    pub volume_throughput: Option<i64>,
}

/// Includes all client authentication related information.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClientAuthentication {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sasl: Option<ClusterClientAuthenticationSasl>,
    /// Details for client authentication using TLS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<ClusterClientAuthenticationTls>,
    /// Contains information about unauthenticated traffic to the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unauthenticated: Option<ClusterClientAuthenticationUnauthenticated>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClientAuthenticationSasl {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iam: Option<ClusterClientAuthenticationSaslIam>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scram: Option<ClusterClientAuthenticationSaslScram>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClientAuthenticationSaslIam {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClientAuthenticationSaslScram {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Details for client authentication using TLS.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClientAuthenticationTls {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificateAuthorityARNList")]
    pub certificate_authority_arn_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Contains information about unauthenticated traffic to the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClientAuthenticationUnauthenticated {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Represents the configuration that you want MSK to use for the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterConfigurationInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}

/// Includes all encryption-related information.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterEncryptionInfo {
    /// The data-volume encryption details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptionAtRest")]
    pub encryption_at_rest: Option<ClusterEncryptionInfoEncryptionAtRest>,
    /// The settings for encrypting data in transit.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptionInTransit")]
    pub encryption_in_transit: Option<ClusterEncryptionInfoEncryptionInTransit>,
}

/// The data-volume encryption details.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterEncryptionInfoEncryptionAtRest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataVolumeKMSKeyID")]
    pub data_volume_kms_key_id: Option<String>,
}

/// The settings for encrypting data in transit.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterEncryptionInfoEncryptionInTransit {
    /// Client-broker encryption in transit setting.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientBroker")]
    pub client_broker: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inCluster")]
    pub in_cluster: Option<bool>,
}

/// LoggingInfo details.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterLoggingInfo {
    /// The broker logs configuration for this MSK cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "brokerLogs")]
    pub broker_logs: Option<ClusterLoggingInfoBrokerLogs>,
}

/// The broker logs configuration for this MSK cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterLoggingInfoBrokerLogs {
    /// Details of the CloudWatch Logs destination for broker logs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cloudWatchLogs")]
    pub cloud_watch_logs: Option<ClusterLoggingInfoBrokerLogsCloudWatchLogs>,
    /// Firehose details for BrokerLogs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firehose: Option<ClusterLoggingInfoBrokerLogsFirehose>,
    /// The details of the Amazon S3 destination for broker logs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<ClusterLoggingInfoBrokerLogsS3>,
}

/// Details of the CloudWatch Logs destination for broker logs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterLoggingInfoBrokerLogsCloudWatchLogs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logGroup")]
    pub log_group: Option<String>,
}

/// Firehose details for BrokerLogs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterLoggingInfoBrokerLogsFirehose {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deliveryStream")]
    pub delivery_stream: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// The details of the Amazon S3 destination for broker logs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterLoggingInfoBrokerLogsS3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// The settings for open monitoring.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOpenMonitoring {
    /// Prometheus settings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prometheus: Option<ClusterOpenMonitoringPrometheus>,
}

/// Prometheus settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOpenMonitoringPrometheus {
    /// Indicates whether you want to enable or disable the JMX Exporter.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jmxExporter")]
    pub jmx_exporter: Option<ClusterOpenMonitoringPrometheusJmxExporter>,
    /// Indicates whether you want to enable or disable the Node Exporter.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeExporter")]
    pub node_exporter: Option<ClusterOpenMonitoringPrometheusNodeExporter>,
}

/// Indicates whether you want to enable or disable the JMX Exporter.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOpenMonitoringPrometheusJmxExporter {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enabledInBroker")]
    pub enabled_in_broker: Option<bool>,
}

/// Indicates whether you want to enable or disable the Node Exporter.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOpenMonitoringPrometheusNodeExporter {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enabledInBroker")]
    pub enabled_in_broker: Option<bool>,
}

/// ClusterStatus defines the observed state of Cluster
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<ClusterStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The state of the cluster. The possible states are ACTIVE, CREATING, DELETING,
    /// FAILED, HEALING, MAINTENANCE, REBOOTING_BROKER, and UPDATING.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "zookeeperConnectString")]
    pub zookeeper_connect_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "zookeeperConnectStringTLS")]
    pub zookeeper_connect_string_tls: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a
    /// globally-unique identifier and is set only by the ACK service controller
    /// once the controller has orchestrated the creation of the resource OR
    /// when it has verified that an "adopted" resource (a resource where the
    /// ARN annotation was set by the Kubernetes user on the CR) exists and
    /// matches the supplied CR's Spec field values.
    /// TODO(vijat@): Find a better strategy for resources that do not have ARN in CreateOutputResponse
    /// https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the
    /// backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

