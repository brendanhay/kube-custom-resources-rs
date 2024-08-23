// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/mariadb-operator/mariadb-operator/k8s.mariadb.com/v1alpha1/grants.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// GrantSpec defines the desired state of Grant
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "k8s.mariadb.com", version = "v1alpha1", kind = "Grant", plural = "grants")]
#[kube(namespaced)]
#[kube(status = "GrantStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct GrantSpec {
    /// CleanupPolicy defines the behavior for cleaning up a SQL resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cleanupPolicy")]
    pub cleanup_policy: Option<GrantCleanupPolicy>,
    /// Database to use in the Grant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// GrantOption to use in the Grant.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "grantOption")]
    pub grant_option: Option<bool>,
    /// Host to use in the Grant. It can be localhost, an IP or '%'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// MariaDBRef is a reference to a MariaDB object.
    #[serde(rename = "mariaDbRef")]
    pub maria_db_ref: GrantMariaDbRef,
    /// Privileges to use in the Grant.
    pub privileges: Vec<String>,
    /// RequeueInterval is used to perform requeue reconciliations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requeueInterval")]
    pub requeue_interval: Option<String>,
    /// RetryInterval is the interval used to perform retries.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryInterval")]
    pub retry_interval: Option<String>,
    /// Table to use in the Grant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    /// Username to use in the Grant.
    pub username: String,
}

/// GrantSpec defines the desired state of Grant
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GrantCleanupPolicy {
    Skip,
    Delete,
}

/// MariaDBRef is a reference to a MariaDB object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrantMariaDbRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// WaitForIt indicates whether the controller using this reference should wait for MariaDB to be ready.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "waitForIt")]
    pub wait_for_it: Option<bool>,
}

/// GrantStatus defines the observed state of Grant
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrantStatus {
    /// Conditions for the Grant object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

