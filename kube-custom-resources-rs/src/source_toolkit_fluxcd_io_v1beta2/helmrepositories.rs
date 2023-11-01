// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/fluxcd/source-controller/source.toolkit.fluxcd.io/v1beta2/helmrepositories.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// HelmRepositorySpec specifies the required configuration to produce an Artifact for a Helm repository index YAML.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "source.toolkit.fluxcd.io", version = "v1beta2", kind = "HelmRepository", plural = "helmrepositories")]
#[kube(namespaced)]
#[kube(status = "HelmRepositoryStatus")]
pub struct HelmRepositorySpec {
    /// AccessFrom specifies an Access Control List for allowing cross-namespace references to this object. NOTE: Not implemented, provisional as of https://github.com/fluxcd/flux2/pull/2092
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessFrom")]
    pub access_from: Option<HelmRepositoryAccessFrom>,
    /// CertSecretRef can be given the name of a Secret containing either or both of 
    ///  - a PEM-encoded client certificate (`tls.crt`) and private key (`tls.key`); - a PEM-encoded CA certificate (`ca.crt`) 
    ///  and whichever are supplied, will be used for connecting to the registry. The client cert and key are useful if you are authenticating with a certificate; the CA cert is useful if you are using a self-signed server certificate. The Secret must be of type `Opaque` or `kubernetes.io/tls`. 
    ///  It takes precedence over the values specified in the Secret referred to by `.spec.secretRef`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certSecretRef")]
    pub cert_secret_ref: Option<HelmRepositoryCertSecretRef>,
    /// Interval at which the HelmRepository URL is checked for updates. This interval is approximate and may be subject to jitter to ensure efficient use of resources.
    pub interval: String,
    /// PassCredentials allows the credentials from the SecretRef to be passed on to a host that does not match the host as defined in URL. This may be required if the host of the advertised chart URLs in the index differ from the defined URL. Enabling this should be done with caution, as it can potentially result in credentials getting stolen in a MITM-attack.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "passCredentials")]
    pub pass_credentials: Option<bool>,
    /// Provider used for authentication, can be 'aws', 'azure', 'gcp' or 'generic'. This field is optional, and only taken into account if the .spec.type field is set to 'oci'. When not specified, defaults to 'generic'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<HelmRepositoryProvider>,
    /// SecretRef specifies the Secret containing authentication credentials for the HelmRepository. For HTTP/S basic auth the secret must contain 'username' and 'password' fields. Support for TLS auth using the 'certFile' and 'keyFile', and/or 'caFile' keys is deprecated. Please use `.spec.certSecretRef` instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<HelmRepositorySecretRef>,
    /// Suspend tells the controller to suspend the reconciliation of this HelmRepository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Timeout is used for the index fetch operation for an HTTPS helm repository, and for remote OCI Repository operations like pulling for an OCI helm repository. Its default value is 60s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    /// Type of the HelmRepository. When this field is set to  "oci", the URL field value must be prefixed with "oci://".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<HelmRepositoryType>,
    /// URL of the Helm repository, a valid URL contains at least a protocol and host.
    pub url: String,
}

/// AccessFrom specifies an Access Control List for allowing cross-namespace references to this object. NOTE: Not implemented, provisional as of https://github.com/fluxcd/flux2/pull/2092
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct HelmRepositoryAccessFrom {
    /// NamespaceSelectors is the list of namespace selectors to which this ACL applies. Items in this list are evaluated using a logical OR operation.
    #[serde(rename = "namespaceSelectors")]
    pub namespace_selectors: Vec<HelmRepositoryAccessFromNamespaceSelectors>,
}

/// NamespaceSelector selects the namespaces to which this ACL applies. An empty map of MatchLabels matches all namespaces in a cluster.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct HelmRepositoryAccessFromNamespaceSelectors {
    /// MatchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// CertSecretRef can be given the name of a Secret containing either or both of 
///  - a PEM-encoded client certificate (`tls.crt`) and private key (`tls.key`); - a PEM-encoded CA certificate (`ca.crt`) 
///  and whichever are supplied, will be used for connecting to the registry. The client cert and key are useful if you are authenticating with a certificate; the CA cert is useful if you are using a self-signed server certificate. The Secret must be of type `Opaque` or `kubernetes.io/tls`. 
///  It takes precedence over the values specified in the Secret referred to by `.spec.secretRef`.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct HelmRepositoryCertSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// HelmRepositorySpec specifies the required configuration to produce an Artifact for a Helm repository index YAML.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum HelmRepositoryProvider {
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "azure")]
    Azure,
    #[serde(rename = "gcp")]
    Gcp,
}

/// SecretRef specifies the Secret containing authentication credentials for the HelmRepository. For HTTP/S basic auth the secret must contain 'username' and 'password' fields. Support for TLS auth using the 'certFile' and 'keyFile', and/or 'caFile' keys is deprecated. Please use `.spec.certSecretRef` instead.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct HelmRepositorySecretRef {
    /// Name of the referent.
    pub name: String,
}

/// HelmRepositorySpec specifies the required configuration to produce an Artifact for a Helm repository index YAML.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum HelmRepositoryType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "oci")]
    Oci,
}

/// HelmRepositoryStatus records the observed state of the HelmRepository.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct HelmRepositoryStatus {
    /// Artifact represents the last successful HelmRepository reconciliation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<HelmRepositoryStatusArtifact>,
    /// Conditions holds the conditions for the HelmRepository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<HelmRepositoryStatusConditions>>,
    /// LastHandledReconcileAt holds the value of the most recent reconcile request value, so a change of the annotation value can be detected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHandledReconcileAt")]
    pub last_handled_reconcile_at: Option<String>,
    /// ObservedGeneration is the last observed generation of the HelmRepository object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// URL is the dynamic fetch link for the latest Artifact. It is provided on a "best effort" basis, and using the precise HelmRepositoryStatus.Artifact data is recommended.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Artifact represents the last successful HelmRepository reconciliation.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct HelmRepositoryStatusArtifact {
    /// Digest is the digest of the file in the form of '<algorithm>:<checksum>'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// LastUpdateTime is the timestamp corresponding to the last update of the Artifact.
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: String,
    /// Metadata holds upstream information such as OCI annotations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    /// Path is the relative file path of the Artifact. It can be used to locate the file in the root of the Artifact storage on the local file system of the controller managing the Source.
    pub path: String,
    /// Revision is a human-readable identifier traceable in the origin source system. It can be a Git commit SHA, Git tag, a Helm chart version, etc.
    pub revision: String,
    /// Size is the number of bytes in the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// URL is the HTTP address of the Artifact as exposed by the controller managing the Source. It can be used to retrieve the Artifact for consumption, e.g. by another controller applying the Artifact contents.
    pub url: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct HelmRepositoryStatusConditions {
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
    pub status: HelmRepositoryStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum HelmRepositoryStatusConditionsStatus {
    True,
    False,
    Unknown,
}

