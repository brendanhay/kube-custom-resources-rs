// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/fluxcd/source-controller/source.toolkit.fluxcd.io/v1beta1/buckets.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// BucketSpec defines the desired state of an S3 compatible bucket
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "source.toolkit.fluxcd.io", version = "v1beta1", kind = "Bucket", plural = "buckets")]
#[kube(namespaced)]
#[kube(status = "BucketStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BucketSpec {
    /// AccessFrom defines an Access Control List for allowing cross-namespace references to this object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessFrom")]
    pub access_from: Option<BucketAccessFrom>,
    /// The bucket name.
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    /// The bucket endpoint address.
    pub endpoint: String,
    /// Ignore overrides the set of excluded patterns in the .sourceignore format
    /// (which is the same as .gitignore). If not provided, a default will be used,
    /// consult the documentation for your version to find out what those are.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore: Option<String>,
    /// Insecure allows connecting to a non-TLS S3 HTTP endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    /// The interval at which to check for bucket updates.
    pub interval: String,
    /// The S3 compatible storage provider name, default ('generic').
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<BucketProvider>,
    /// The bucket region.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The name of the secret containing authentication credentials
    /// for the Bucket.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<BucketSecretRef>,
    /// This flag tells the controller to suspend the reconciliation of this source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// The timeout for download operations, defaults to 60s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// AccessFrom defines an Access Control List for allowing cross-namespace references to this object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BucketAccessFrom {
    /// NamespaceSelectors is the list of namespace selectors to which this ACL applies.
    /// Items in this list are evaluated using a logical OR operation.
    #[serde(rename = "namespaceSelectors")]
    pub namespace_selectors: Vec<BucketAccessFromNamespaceSelectors>,
}

/// NamespaceSelector selects the namespaces to which this ACL applies.
/// An empty map of MatchLabels matches all namespaces in a cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BucketAccessFromNamespaceSelectors {
    /// MatchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// BucketSpec defines the desired state of an S3 compatible bucket
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BucketProvider {
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "gcp")]
    Gcp,
}

/// The name of the secret containing authentication credentials
/// for the Bucket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BucketSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// BucketStatus defines the observed state of a bucket
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BucketStatus {
    /// Artifact represents the output of the last successful Bucket sync.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<BucketStatusArtifact>,
    /// Conditions holds the conditions for the Bucket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// LastHandledReconcileAt holds the value of the most recent
    /// reconcile request value, so a change of the annotation value
    /// can be detected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHandledReconcileAt")]
    pub last_handled_reconcile_at: Option<String>,
    /// ObservedGeneration is the last observed generation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// URL is the download link for the artifact output of the last Bucket sync.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Artifact represents the output of the last successful Bucket sync.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BucketStatusArtifact {
    /// Checksum is the SHA256 checksum of the artifact.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// LastUpdateTime is the timestamp corresponding to the last update of this
    /// artifact.
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: String,
    /// Path is the relative file path of this artifact.
    pub path: String,
    /// Revision is a human readable identifier traceable in the origin source
    /// system. It can be a Git commit SHA, Git tag, a Helm index timestamp, a Helm
    /// chart version, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// URL is the HTTP address of this artifact.
    pub url: String,
}

