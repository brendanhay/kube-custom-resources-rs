// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/fluxcd/notification-controller/notification.toolkit.fluxcd.io/v1beta2/providers.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// ProviderSpec defines the desired state of the Provider.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "notification.toolkit.fluxcd.io", version = "v1beta2", kind = "Provider", plural = "providers")]
#[kube(namespaced)]
#[kube(status = "ProviderStatus")]
#[kube(schema = "disabled")]
pub struct ProviderSpec {
    /// Address specifies the endpoint, in a generic sense, to where alerts are sent. What kind of endpoint depends on the specific Provider type being used. For the generic Provider, for example, this is an HTTP/S address. For other Provider types this could be a project ID or a namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// CertSecretRef specifies the Secret containing a PEM-encoded CA certificate (in the `ca.crt` key). 
    ///  Note: Support for the `caFile` key has been deprecated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certSecretRef")]
    pub cert_secret_ref: Option<ProviderCertSecretRef>,
    /// Channel specifies the destination channel where events should be posted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// Interval at which to reconcile the Provider with its Secret references.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Proxy the HTTP/S address of the proxy server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    /// SecretRef specifies the Secret containing the authentication credentials for this Provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<ProviderSecretRef>,
    /// Suspend tells the controller to suspend subsequent events handling for this Provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Timeout for sending alerts to the Provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    /// Type specifies which Provider implementation to use.
    #[serde(rename = "type")]
    pub r#type: ProviderType,
    /// Username specifies the name under which events are posted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// CertSecretRef specifies the Secret containing a PEM-encoded CA certificate (in the `ca.crt` key). 
///  Note: Support for the `caFile` key has been deprecated.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProviderCertSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// SecretRef specifies the Secret containing the authentication credentials for this Provider.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProviderSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// ProviderSpec defines the desired state of the Provider.
#[derive(Serialize, Deserialize, Clone, Debug)]
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
    #[serde(rename = "gitea")]
    Gitea,
    #[serde(rename = "bitbucket")]
    Bitbucket,
    #[serde(rename = "azuredevops")]
    Azuredevops,
    #[serde(rename = "googlechat")]
    Googlechat,
    #[serde(rename = "googlepubsub")]
    Googlepubsub,
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
    #[serde(rename = "pagerduty")]
    Pagerduty,
    #[serde(rename = "datadog")]
    Datadog,
}

/// ProviderStatus defines the observed state of the Provider.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProviderStatus {
    /// Conditions holds the conditions for the Provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ProviderStatusConditions>>,
    /// LastHandledReconcileAt holds the value of the most recent reconcile request value, so a change of the annotation value can be detected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHandledReconcileAt")]
    pub last_handled_reconcile_at: Option<String>,
    /// ObservedGeneration is the last reconciled generation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug)]
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
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ProviderStatusConditionsStatus {
    True,
    False,
    Unknown,
}

