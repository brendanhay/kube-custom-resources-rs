// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/fluxcd/source-controller/source.toolkit.fluxcd.io/v1beta2/ocirepositories.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// OCIRepositorySpec defines the desired state of OCIRepository
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "source.toolkit.fluxcd.io", version = "v1beta2", kind = "OCIRepository", plural = "ocirepositories")]
#[kube(namespaced)]
#[kube(status = "OCIRepositoryStatus")]
pub struct OCIRepositorySpec {
    /// CertSecretRef can be given the name of a Secret containing either or both of 
    ///  - a PEM-encoded client certificate (`tls.crt`) and private key (`tls.key`); - a PEM-encoded CA certificate (`ca.crt`) 
    ///  and whichever are supplied, will be used for connecting to the registry. The client cert and key are useful if you are authenticating with a certificate; the CA cert is useful if you are using a self-signed server certificate. The Secret must be of type `Opaque` or `kubernetes.io/tls`. 
    ///  Note: Support for the `caFile`, `certFile` and `keyFile` keys have been deprecated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certSecretRef")]
    pub cert_secret_ref: Option<OCIRepositoryCertSecretRef>,
    /// Ignore overrides the set of excluded patterns in the .sourceignore format (which is the same as .gitignore). If not provided, a default will be used, consult the documentation for your version to find out what those are.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore: Option<String>,
    /// Insecure allows connecting to a non-TLS HTTP container registry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    /// Interval at which the OCIRepository URL is checked for updates. This interval is approximate and may be subject to jitter to ensure efficient use of resources.
    pub interval: String,
    /// LayerSelector specifies which layer should be extracted from the OCI artifact. When not specified, the first layer found in the artifact is selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "layerSelector")]
    pub layer_selector: Option<OCIRepositoryLayerSelector>,
    /// The provider used for authentication, can be 'aws', 'azure', 'gcp' or 'generic'. When not specified, defaults to 'generic'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<OCIRepositoryProvider>,
    /// The OCI reference to pull and monitor for changes, defaults to the latest tag.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ref")]
    pub r#ref: Option<OCIRepositoryRef>,
    /// SecretRef contains the secret name containing the registry login credentials to resolve image metadata. The secret must be of type kubernetes.io/dockerconfigjson.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<OCIRepositorySecretRef>,
    /// ServiceAccountName is the name of the Kubernetes ServiceAccount used to authenticate the image pull if the service account has attached pull secrets. For more information: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/#add-imagepullsecrets-to-a-service-account
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccountName")]
    pub service_account_name: Option<String>,
    /// This flag tells the controller to suspend the reconciliation of this source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// The timeout for remote OCI Repository operations like pulling, defaults to 60s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    /// URL is a reference to an OCI artifact repository hosted on a remote container registry.
    pub url: String,
    /// Verify contains the secret name containing the trusted public keys used to verify the signature and specifies which provider to use to check whether OCI image is authentic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verify: Option<OCIRepositoryVerify>,
}

/// CertSecretRef can be given the name of a Secret containing either or both of 
///  - a PEM-encoded client certificate (`tls.crt`) and private key (`tls.key`); - a PEM-encoded CA certificate (`ca.crt`) 
///  and whichever are supplied, will be used for connecting to the registry. The client cert and key are useful if you are authenticating with a certificate; the CA cert is useful if you are using a self-signed server certificate. The Secret must be of type `Opaque` or `kubernetes.io/tls`. 
///  Note: Support for the `caFile`, `certFile` and `keyFile` keys have been deprecated.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct OCIRepositoryCertSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// LayerSelector specifies which layer should be extracted from the OCI artifact. When not specified, the first layer found in the artifact is selected.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct OCIRepositoryLayerSelector {
    /// MediaType specifies the OCI media type of the layer which should be extracted from the OCI Artifact. The first layer matching this type is selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mediaType")]
    pub media_type: Option<String>,
    /// Operation specifies how the selected layer should be processed. By default, the layer compressed content is extracted to storage. When the operation is set to 'copy', the layer compressed content is persisted to storage as it is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<OCIRepositoryLayerSelectorOperation>,
}

/// LayerSelector specifies which layer should be extracted from the OCI artifact. When not specified, the first layer found in the artifact is selected.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum OCIRepositoryLayerSelectorOperation {
    #[serde(rename = "extract")]
    Extract,
    #[serde(rename = "copy")]
    Copy,
}

/// OCIRepositorySpec defines the desired state of OCIRepository
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum OCIRepositoryProvider {
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "azure")]
    Azure,
    #[serde(rename = "gcp")]
    Gcp,
}

/// The OCI reference to pull and monitor for changes, defaults to the latest tag.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct OCIRepositoryRef {
    /// Digest is the image digest to pull, takes precedence over SemVer. The value should be in the format 'sha256:<HASH>'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// SemVer is the range of tags to pull selecting the latest within the range, takes precedence over Tag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semver: Option<String>,
    /// Tag is the image tag to pull, defaults to latest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// SecretRef contains the secret name containing the registry login credentials to resolve image metadata. The secret must be of type kubernetes.io/dockerconfigjson.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct OCIRepositorySecretRef {
    /// Name of the referent.
    pub name: String,
}

/// Verify contains the secret name containing the trusted public keys used to verify the signature and specifies which provider to use to check whether OCI image is authentic.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct OCIRepositoryVerify {
    /// Provider specifies the technology used to sign the OCI Artifact.
    pub provider: OCIRepositoryVerifyProvider,
    /// SecretRef specifies the Kubernetes Secret containing the trusted public keys.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<OCIRepositoryVerifySecretRef>,
}

/// Verify contains the secret name containing the trusted public keys used to verify the signature and specifies which provider to use to check whether OCI image is authentic.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum OCIRepositoryVerifyProvider {
    #[serde(rename = "cosign")]
    Cosign,
}

/// SecretRef specifies the Kubernetes Secret containing the trusted public keys.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct OCIRepositoryVerifySecretRef {
    /// Name of the referent.
    pub name: String,
}

/// OCIRepositoryStatus defines the observed state of OCIRepository
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct OCIRepositoryStatus {
    /// Artifact represents the output of the last successful OCI Repository sync.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<OCIRepositoryStatusArtifact>,
    /// Conditions holds the conditions for the OCIRepository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<OCIRepositoryStatusConditions>>,
    /// ContentConfigChecksum is a checksum of all the configurations related to the content of the source artifact: - .spec.ignore - .spec.layerSelector observed in .status.observedGeneration version of the object. This can be used to determine if the content configuration has changed and the artifact needs to be rebuilt. It has the format of `<algo>:<checksum>`, for example: `sha256:<checksum>`. 
    ///  Deprecated: Replaced with explicit fields for observed artifact content config in the status.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contentConfigChecksum")]
    pub content_config_checksum: Option<String>,
    /// LastHandledReconcileAt holds the value of the most recent reconcile request value, so a change of the annotation value can be detected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHandledReconcileAt")]
    pub last_handled_reconcile_at: Option<String>,
    /// ObservedGeneration is the last observed generation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// ObservedIgnore is the observed exclusion patterns used for constructing the source artifact.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedIgnore")]
    pub observed_ignore: Option<String>,
    /// ObservedLayerSelector is the observed layer selector used for constructing the source artifact.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedLayerSelector")]
    pub observed_layer_selector: Option<OCIRepositoryStatusObservedLayerSelector>,
    /// URL is the download link for the artifact output of the last OCI Repository sync.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Artifact represents the output of the last successful OCI Repository sync.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct OCIRepositoryStatusArtifact {
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
pub struct OCIRepositoryStatusConditions {
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
    pub status: OCIRepositoryStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum OCIRepositoryStatusConditionsStatus {
    True,
    False,
    Unknown,
}

/// ObservedLayerSelector is the observed layer selector used for constructing the source artifact.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct OCIRepositoryStatusObservedLayerSelector {
    /// MediaType specifies the OCI media type of the layer which should be extracted from the OCI Artifact. The first layer matching this type is selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mediaType")]
    pub media_type: Option<String>,
    /// Operation specifies how the selected layer should be processed. By default, the layer compressed content is extracted to storage. When the operation is set to 'copy', the layer compressed content is persisted to storage as it is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<OCIRepositoryStatusObservedLayerSelectorOperation>,
}

/// ObservedLayerSelector is the observed layer selector used for constructing the source artifact.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum OCIRepositoryStatusObservedLayerSelectorOperation {
    #[serde(rename = "extract")]
    Extract,
    #[serde(rename = "copy")]
    Copy,
}

