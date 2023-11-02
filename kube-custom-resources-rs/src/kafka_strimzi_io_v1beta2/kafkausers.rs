// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/strimzi/strimzi-kafka-operator/kafka.strimzi.io/v1beta2/kafkausers.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// The specification of the user.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "kafka.strimzi.io", version = "v1beta2", kind = "KafkaUser", plural = "kafkausers")]
#[kube(namespaced)]
#[kube(status = "KafkaUserStatus")]
#[kube(schema = "disabled")]
pub struct KafkaUserSpec {
    /// Authentication mechanism enabled for this Kafka user. The supported authentication mechanisms are `scram-sha-512`, `tls`, and `tls-external`. 
    /// 
    /// * `scram-sha-512` generates a secret with SASL SCRAM-SHA-512 credentials.
    /// * `tls` generates a secret with user certificate for mutual TLS authentication.
    /// * `tls-external` does not generate a user certificate.   But prepares the user for using mutual TLS authentication using a user certificate generated outside the User Operator.
    ///   ACLs and quotas set for this user are configured in the `CN=<username>` format.
    /// 
    /// Authentication is optional. If authentication is not configured, no credentials are generated. ACLs and quotas set for the user are configured in the `<username>` format suitable for SASL authentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<KafkaUserAuthentication>,
    /// Authorization rules for this Kafka user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<KafkaUserAuthorization>,
    /// Quotas on requests to control the broker resources used by clients. Network bandwidth and request rate quotas can be enforced.Kafka documentation for Kafka User quotas can be found at http://kafka.apache.org/documentation/#design_quotas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quotas: Option<KafkaUserQuotas>,
    /// Template to specify how Kafka User `Secrets` are generated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<KafkaUserTemplate>,
}

/// Authentication mechanism enabled for this Kafka user. The supported authentication mechanisms are `scram-sha-512`, `tls`, and `tls-external`. 
/// 
/// * `scram-sha-512` generates a secret with SASL SCRAM-SHA-512 credentials.
/// * `tls` generates a secret with user certificate for mutual TLS authentication.
/// * `tls-external` does not generate a user certificate.   But prepares the user for using mutual TLS authentication using a user certificate generated outside the User Operator.
///   ACLs and quotas set for this user are configured in the `CN=<username>` format.
/// 
/// Authentication is optional. If authentication is not configured, no credentials are generated. ACLs and quotas set for the user are configured in the `<username>` format suitable for SASL authentication.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KafkaUserAuthentication {
    /// Specify the password for the user. If not set, a new password is generated by the User Operator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<KafkaUserAuthenticationPassword>,
    /// Authentication type.
    #[serde(rename = "type")]
    pub r#type: KafkaUserAuthenticationType,
}

/// Specify the password for the user. If not set, a new password is generated by the User Operator.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KafkaUserAuthenticationPassword {
    /// Secret from which the password should be read.
    #[serde(rename = "valueFrom")]
    pub value_from: KafkaUserAuthenticationPasswordValueFrom,
}

/// Secret from which the password should be read.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KafkaUserAuthenticationPasswordValueFrom {
    /// Selects a key of a Secret in the resource's namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<KafkaUserAuthenticationPasswordValueFromSecretKeyRef>,
}

/// Selects a key of a Secret in the resource's namespace.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KafkaUserAuthenticationPasswordValueFromSecretKeyRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Authentication mechanism enabled for this Kafka user. The supported authentication mechanisms are `scram-sha-512`, `tls`, and `tls-external`. 
/// 
/// * `scram-sha-512` generates a secret with SASL SCRAM-SHA-512 credentials.
/// * `tls` generates a secret with user certificate for mutual TLS authentication.
/// * `tls-external` does not generate a user certificate.   But prepares the user for using mutual TLS authentication using a user certificate generated outside the User Operator.
///   ACLs and quotas set for this user are configured in the `CN=<username>` format.
/// 
/// Authentication is optional. If authentication is not configured, no credentials are generated. ACLs and quotas set for the user are configured in the `<username>` format suitable for SASL authentication.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum KafkaUserAuthenticationType {
    #[serde(rename = "tls")]
    Tls,
    #[serde(rename = "tls-external")]
    TlsExternal,
    #[serde(rename = "scram-sha-512")]
    ScramSha512,
}

/// Authorization rules for this Kafka user.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KafkaUserAuthorization {
    /// List of ACL rules which should be applied to this user.
    pub acls: Vec<KafkaUserAuthorizationAcls>,
    /// Authorization type. Currently the only supported type is `simple`. `simple` authorization type uses the Kafka Admin API for managing the ACL rules.
    #[serde(rename = "type")]
    pub r#type: KafkaUserAuthorizationType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KafkaUserAuthorizationAcls {
    /// The host from which the action described in the ACL rule is allowed or denied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Operation which will be allowed or denied. Supported operations are: Read, Write, Create, Delete, Alter, Describe, ClusterAction, AlterConfigs, DescribeConfigs, IdempotentWrite and All.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<KafkaUserAuthorizationAclsOperation>,
    /// List of operations which will be allowed or denied. Supported operations are: Read, Write, Create, Delete, Alter, Describe, ClusterAction, AlterConfigs, DescribeConfigs, IdempotentWrite and All.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
    /// Indicates the resource for which given ACL rule applies.
    pub resource: KafkaUserAuthorizationAclsResource,
    /// The type of the rule. Currently the only supported type is `allow`. ACL rules with type `allow` are used to allow user to execute the specified operations. Default value is `allow`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<KafkaUserAuthorizationAclsType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum KafkaUserAuthorizationAclsOperation {
    Read,
    Write,
    Create,
    Delete,
    Alter,
    Describe,
    ClusterAction,
    AlterConfigs,
    DescribeConfigs,
    IdempotentWrite,
    All,
}

/// Indicates the resource for which given ACL rule applies.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KafkaUserAuthorizationAclsResource {
    /// Name of resource for which given ACL rule applies. Can be combined with `patternType` field to use prefix pattern.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Describes the pattern used in the resource field. The supported types are `literal` and `prefix`. With `literal` pattern type, the resource field will be used as a definition of a full name. With `prefix` pattern type, the resource name will be used only as a prefix. Default value is `literal`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "patternType")]
    pub pattern_type: Option<KafkaUserAuthorizationAclsResourcePatternType>,
    /// Resource type. The available resource types are `topic`, `group`, `cluster`, and `transactionalId`.
    #[serde(rename = "type")]
    pub r#type: KafkaUserAuthorizationAclsResourceType,
}

/// Indicates the resource for which given ACL rule applies.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum KafkaUserAuthorizationAclsResourcePatternType {
    #[serde(rename = "literal")]
    Literal,
    #[serde(rename = "prefix")]
    Prefix,
}

/// Indicates the resource for which given ACL rule applies.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum KafkaUserAuthorizationAclsResourceType {
    #[serde(rename = "topic")]
    Topic,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "cluster")]
    Cluster,
    #[serde(rename = "transactionalId")]
    TransactionalId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum KafkaUserAuthorizationAclsType {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}

/// Authorization rules for this Kafka user.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum KafkaUserAuthorizationType {
    #[serde(rename = "simple")]
    Simple,
}

/// Quotas on requests to control the broker resources used by clients. Network bandwidth and request rate quotas can be enforced.Kafka documentation for Kafka User quotas can be found at http://kafka.apache.org/documentation/#design_quotas.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KafkaUserQuotas {
    /// A quota on the maximum bytes per-second that each client group can fetch from a broker before the clients in the group are throttled. Defined on a per-broker basis.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "consumerByteRate")]
    pub consumer_byte_rate: Option<i64>,
    /// A quota on the rate at which mutations are accepted for the create topics request, the create partitions request and the delete topics request. The rate is accumulated by the number of partitions created or deleted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controllerMutationRate")]
    pub controller_mutation_rate: Option<f64>,
    /// A quota on the maximum bytes per-second that each client group can publish to a broker before the clients in the group are throttled. Defined on a per-broker basis.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "producerByteRate")]
    pub producer_byte_rate: Option<i64>,
    /// A quota on the maximum CPU utilization of each client group as a percentage of network and I/O threads.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestPercentage")]
    pub request_percentage: Option<i64>,
}

/// Template to specify how Kafka User `Secrets` are generated.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KafkaUserTemplate {
    /// Template for KafkaUser resources. The template allows users to specify how the `Secret` with password or TLS certificates is generated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<KafkaUserTemplateSecret>,
}

/// Template for KafkaUser resources. The template allows users to specify how the `Secret` with password or TLS certificates is generated.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KafkaUserTemplateSecret {
    /// Metadata applied to the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<KafkaUserTemplateSecretMetadata>,
}

/// Metadata applied to the resource.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KafkaUserTemplateSecretMetadata {
    /// Annotations added to the Kubernetes resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, serde_json::Value>>,
    /// Labels added to the Kubernetes resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, serde_json::Value>>,
}

/// The status of the Kafka User.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KafkaUserStatus {
    /// List of status conditions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<KafkaUserStatusConditions>>,
    /// The generation of the CRD that was last reconciled by the operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// The name of `Secret` where the credentials are stored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Username.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KafkaUserStatusConditions {
    /// Last time the condition of a type changed from one status to another. The required format is 'yyyy-MM-ddTHH:mm:ssZ', in the UTC time zone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// Human-readable message indicating details about the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition (a single word in CamelCase).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The status of the condition, either True, False or Unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The unique identifier of a condition, used to distinguish between other conditions in the resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

