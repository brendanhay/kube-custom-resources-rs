// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/fluxcd/notification-controller/notification.toolkit.fluxcd.io/v1beta1/providers.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// ProviderSpec defines the desired state of Provider
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "notification.toolkit.fluxcd.io", version = "v1beta1", kind = "Provider", plural = "providers")]
#[kube(namespaced)]
#[kube(status = "ProviderStatus")]
pub struct ProviderSpec {
    /// HTTP/S webhook address of this provider
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// CertSecretRef can be given the name of a secret containing a PEM-encoded CA certificate (`caFile`)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certSecretRef")]
    pub cert_secret_ref: Option<ProviderCertSecretRef>,
    /// Alert channel for this provider
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// HTTP/S address of the proxy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    /// Secret reference containing the provider webhook URL using "address" as data key
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<ProviderSecretRef>,
    /// This flag tells the controller to suspend subsequent events handling. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Timeout for sending alerts to the provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    /// Type of provider
    #[serde(rename = "type")]
    pub r#type: ProviderType,
    /// Bot username for this provider
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// CertSecretRef can be given the name of a secret containing a PEM-encoded CA certificate (`caFile`)
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ProviderCertSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// Secret reference containing the provider webhook URL using "address" as data key
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ProviderSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// ProviderSpec defines the desired state of Provider
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum ProviderType {
    #[serde(rename = "slack")]
    Slack,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "msteams")]
    Msteams,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "generic-hmac")]
    GenericHmac,
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "gitlab")]
    Gitlab,
    #[serde(rename = "bitbucket")]
    Bitbucket,
    #[serde(rename = "azuredevops")]
    Azuredevops,
    #[serde(rename = "googlechat")]
    Googlechat,
    #[serde(rename = "webex")]
    Webex,
    #[serde(rename = "sentry")]
    Sentry,
    #[serde(rename = "azureeventhub")]
    Azureeventhub,
    #[serde(rename = "telegram")]
    Telegram,
    #[serde(rename = "lark")]
    Lark,
    #[serde(rename = "matrix")]
    Matrix,
    #[serde(rename = "opsgenie")]
    Opsgenie,
    #[serde(rename = "alertmanager")]
    Alertmanager,
    #[serde(rename = "grafana")]
    Grafana,
    #[serde(rename = "githubdispatch")]
    Githubdispatch,
}

/// ProviderStatus defines the observed state of Provider
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ProviderStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ProviderStatusConditions>>,
    /// ObservedGeneration is the last reconciled generation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ProviderStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: ProviderStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum ProviderStatusConditionsStatus {
    True,
    False,
    Unknown,
}

