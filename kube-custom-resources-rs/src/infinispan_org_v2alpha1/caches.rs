// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/infinispan/infinispan-operator/infinispan.org/v2alpha1/caches.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// CacheSpec defines the desired state of Cache
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infinispan.org", version = "v2alpha1", kind = "Cache", plural = "caches")]
#[kube(namespaced)]
#[kube(status = "CacheStatus")]
#[kube(schema = "disabled")]
pub struct CacheSpec {
    /// Deprecated. This no longer has any effect. The operator's admin credentials are now used to perform cache operations
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminAuth")]
    pub admin_auth: Option<CacheAdminAuth>,
    /// Infinispan cluster name
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// Name of the cache to be created. If empty ObjectMeta.Name will be used
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Cache template in XML format
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// Name of the template to be used to create this cache
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateName")]
    pub template_name: Option<String>,
    /// How updates to Cache CR template should be reconciled on the Infinispan server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updates: Option<CacheUpdates>,
}

/// Deprecated. This no longer has any effect. The operator's admin credentials are now used to perform cache operations
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheAdminAuth {
    /// Secret and key containing the admin password for authentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<CacheAdminAuthPassword>,
    /// The secret that contains user credentials.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
    /// Secret and key containing the admin username for authentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<CacheAdminAuthUsername>,
}

/// Secret and key containing the admin password for authentication.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheAdminAuthPassword {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Secret and key containing the admin username for authentication.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheAdminAuthUsername {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// How updates to Cache CR template should be reconciled on the Infinispan server
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheUpdates {
    /// How updates to Cache CR template should be applied on the Infinispan server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<CacheUpdatesStrategy>,
}

/// How updates to Cache CR template should be reconciled on the Infinispan server
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CacheUpdatesStrategy {
    #[serde(rename = "recreate")]
    Recreate,
    #[serde(rename = "retain")]
    Retain,
}

/// CacheStatus defines the observed state of Cache
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheStatus {
    /// Conditions list for this cache
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<CacheStatusConditions>>,
    /// Deprecated. This is no longer set. Service name that exposes the cache inside the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceName")]
    pub service_name: Option<String>,
}

/// CacheCondition define a condition of the cluster
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CacheStatusConditions {
    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Status is the status of the condition.
    pub status: String,
    /// Type is the type of the condition.
    #[serde(rename = "type")]
    pub r#type: String,
}

