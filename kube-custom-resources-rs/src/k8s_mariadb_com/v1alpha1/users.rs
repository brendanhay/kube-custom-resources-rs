// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/mariadb-operator/mariadb-operator/k8s.mariadb.com/v1alpha1/users.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// UserSpec defines the desired state of User
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "k8s.mariadb.com", version = "v1alpha1", kind = "User", plural = "users")]
#[kube(namespaced)]
#[kube(status = "UserStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct UserSpec {
    /// CleanupPolicy defines the behavior for cleaning up a SQL resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cleanupPolicy")]
    pub cleanup_policy: Option<UserCleanupPolicy>,
    /// Host related to the User.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// MariaDBRef is a reference to a MariaDB object.
    #[serde(rename = "mariaDbRef")]
    pub maria_db_ref: UserMariaDbRef,
    /// MaxUserConnections defines the maximum number of connections that the User can establish.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUserConnections")]
    pub max_user_connections: Option<i32>,
    /// Name overrides the default name provided by metadata.name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// PasswordHashSecretKeyRef is a reference to the password hash to be used by the User.
    /// If the referred Secret is labeled with "k8s.mariadb.com/watch", updates may be performed to the Secret in order to update the password hash.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "passwordHashSecretKeyRef")]
    pub password_hash_secret_key_ref: Option<UserPasswordHashSecretKeyRef>,
    /// PasswordPlugin is a reference to the password plugin and arguments to be used by the User.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "passwordPlugin")]
    pub password_plugin: Option<UserPasswordPlugin>,
    /// PasswordSecretKeyRef is a reference to the password to be used by the User.
    /// If not provided, the account will be locked and the password will expire.
    /// If the referred Secret is labeled with "k8s.mariadb.com/watch", updates may be performed to the Secret in order to update the password.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "passwordSecretKeyRef")]
    pub password_secret_key_ref: Option<UserPasswordSecretKeyRef>,
    /// RequeueInterval is used to perform requeue reconciliations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requeueInterval")]
    pub requeue_interval: Option<String>,
    /// RetryInterval is the interval used to perform retries.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryInterval")]
    pub retry_interval: Option<String>,
}

/// UserSpec defines the desired state of User
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UserCleanupPolicy {
    Skip,
    Delete,
}

/// MariaDBRef is a reference to a MariaDB object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserMariaDbRef {
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

/// PasswordHashSecretKeyRef is a reference to the password hash to be used by the User.
/// If the referred Secret is labeled with "k8s.mariadb.com/watch", updates may be performed to the Secret in order to update the password hash.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserPasswordHashSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// PasswordPlugin is a reference to the password plugin and arguments to be used by the User.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserPasswordPlugin {
    /// PluginArgSecretKeyRef is a reference to the arguments to be provided to the authentication plugin for the User.
    /// If the referred Secret is labeled with "k8s.mariadb.com/watch", updates may be performed to the Secret in order to update the authentication plugin arguments.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pluginArgSecretKeyRef")]
    pub plugin_arg_secret_key_ref: Option<UserPasswordPluginPluginArgSecretKeyRef>,
    /// PluginNameSecretKeyRef is a reference to the authentication plugin to be used by the User.
    /// If the referred Secret is labeled with "k8s.mariadb.com/watch", updates may be performed to the Secret in order to update the authentication plugin.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pluginNameSecretKeyRef")]
    pub plugin_name_secret_key_ref: Option<UserPasswordPluginPluginNameSecretKeyRef>,
}

/// PluginArgSecretKeyRef is a reference to the arguments to be provided to the authentication plugin for the User.
/// If the referred Secret is labeled with "k8s.mariadb.com/watch", updates may be performed to the Secret in order to update the authentication plugin arguments.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserPasswordPluginPluginArgSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// PluginNameSecretKeyRef is a reference to the authentication plugin to be used by the User.
/// If the referred Secret is labeled with "k8s.mariadb.com/watch", updates may be performed to the Secret in order to update the authentication plugin.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserPasswordPluginPluginNameSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// PasswordSecretKeyRef is a reference to the password to be used by the User.
/// If not provided, the account will be locked and the password will expire.
/// If the referred Secret is labeled with "k8s.mariadb.com/watch", updates may be performed to the Secret in order to update the password.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserPasswordSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// UserStatus defines the observed state of User
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UserStatus {
    /// Conditions for the User object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

