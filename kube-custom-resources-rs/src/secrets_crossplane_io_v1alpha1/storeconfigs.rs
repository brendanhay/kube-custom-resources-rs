// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/crossplane/crossplane/secrets.crossplane.io/v1alpha1/storeconfigs.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// A StoreConfigSpec defines the desired state of a StoreConfig.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "secrets.crossplane.io", version = "v1alpha1", kind = "StoreConfig", plural = "storeconfigs")]
pub struct StoreConfigSpec {
    /// DefaultScope used for scoping secrets for "cluster-scoped" resources. If store type is "Kubernetes", this would mean the default namespace to store connection secrets for cluster scoped resources. In case of "Vault", this would be used as the default parent path. Typically, should be set as Crossplane installation namespace.
    #[serde(rename = "defaultScope")]
    pub default_scope: String,
    /// Kubernetes configures a Kubernetes secret store. If the "type" is "Kubernetes" but no config provided, in cluster config will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubernetes: Option<StoreConfigKubernetes>,
    /// Plugin configures External secret store as a plugin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin: Option<StoreConfigPlugin>,
    /// Type configures which secret store to be used. Only the configuration block for this store will be used and others will be ignored if provided. Default is Kubernetes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<StoreConfigType>,
}

/// Kubernetes configures a Kubernetes secret store. If the "type" is "Kubernetes" but no config provided, in cluster config will be used.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct StoreConfigKubernetes {
    /// Credentials used to connect to the Kubernetes API.
    pub auth: StoreConfigKubernetesAuth,
}

/// Credentials used to connect to the Kubernetes API.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct StoreConfigKubernetesAuth {
    /// Env is a reference to an environment variable that contains credentials that must be used to connect to the provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<StoreConfigKubernetesAuthEnv>,
    /// Fs is a reference to a filesystem location that contains credentials that must be used to connect to the provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs: Option<StoreConfigKubernetesAuthFs>,
    /// A SecretRef is a reference to a secret key that contains the credentials that must be used to connect to the provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<StoreConfigKubernetesAuthSecretRef>,
    /// Source of the credentials.
    pub source: StoreConfigKubernetesAuthSource,
}

/// Env is a reference to an environment variable that contains credentials that must be used to connect to the provider.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct StoreConfigKubernetesAuthEnv {
    /// Name is the name of an environment variable.
    pub name: String,
}

/// Fs is a reference to a filesystem location that contains credentials that must be used to connect to the provider.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct StoreConfigKubernetesAuthFs {
    /// Path is a filesystem path.
    pub path: String,
}

/// A SecretRef is a reference to a secret key that contains the credentials that must be used to connect to the provider.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct StoreConfigKubernetesAuthSecretRef {
    /// The key to select.
    pub key: String,
    /// Name of the secret.
    pub name: String,
    /// Namespace of the secret.
    pub namespace: String,
}

/// Credentials used to connect to the Kubernetes API.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum StoreConfigKubernetesAuthSource {
    None,
    Secret,
    Environment,
    Filesystem,
}

/// Plugin configures External secret store as a plugin.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct StoreConfigPlugin {
    /// ConfigRef contains store config reference info.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configRef")]
    pub config_ref: Option<StoreConfigPluginConfigRef>,
    /// Endpoint is the endpoint of the gRPC server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}

/// ConfigRef contains store config reference info.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct StoreConfigPluginConfigRef {
    /// APIVersion of the referenced config.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind of the referenced config.
    pub kind: String,
    /// Name of the referenced config.
    pub name: String,
}

/// A StoreConfigSpec defines the desired state of a StoreConfig.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum StoreConfigType {
    Kubernetes,
    Vault,
    Plugin,
}

