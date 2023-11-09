// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/ktsstudio/mirrors/mirrors.kts.studio/v1alpha2/secretmirrors.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// SecretMirrorSpec defines the desired behaviour of Secret mirroring
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "mirrors.kts.studio", version = "v1alpha2", kind = "SecretMirror", plural = "secretmirrors")]
#[kube(namespaced)]
#[kube(status = "SecretMirrorStatus")]
#[kube(schema = "disabled")]
pub struct SecretMirrorSpec {
    /// What to do with Secret objects created by a SecretMirror. Two policies exist – delete (deletes all created secrets) and retain (leaves them in the cluster). Default: delete
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deletePolicy")]
    pub delete_policy: Option<SecretMirrorDeletePolicy>,
    /// SecretMirrorDestination defines where to sync a secret data to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<SecretMirrorDestination>,
    /// How often to check for secret changes. Default: 180 seconds
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pollPeriodSeconds")]
    pub poll_period_seconds: Option<i64>,
    /// SecretMirrorSource defines where to extract a secret data from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<SecretMirrorSource>,
}

/// SecretMirrorSpec defines the desired behaviour of Secret mirroring
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecretMirrorDeletePolicy {
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "retain")]
    Retain,
}

/// SecretMirrorDestination defines where to sync a secret data to
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorDestination {
    /// An array of regular expressions to match namespaces where to copy a source secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// Destination type. Possible values — namespaces, vault. Default: namespaces
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<SecretMirrorDestinationType>,
    /// VaultSpec contains information of secret location
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vault: Option<SecretMirrorDestinationVault>,
}

/// SecretMirrorDestination defines where to sync a secret data to
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecretMirrorDestinationType {
    #[serde(rename = "namespaces")]
    Namespaces,
    #[serde(rename = "vault")]
    Vault,
}

/// VaultSpec contains information of secret location
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorDestinationVault {
    /// Addr specifies a Vault endpoint URL (e.g. https://vault.example.com)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    /// VaultAuthSpec describes how to authenticate against a Vault server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<SecretMirrorDestinationVaultAuth>,
    /// Path specifies a vault secret path (e.g. secret/data/some-secret or mongodb/creds/mymongo)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// VaultAuthSpec describes how to authenticate against a Vault server
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorDestinationVaultAuth {
    /// VaultAppRoleAuthSpec specifies approle-specific auth data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approle: Option<SecretMirrorDestinationVaultAuthApprole>,
    /// VaultTokenAuthSpec specifies token-specific auth data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<SecretMirrorDestinationVaultAuthToken>,
}

/// VaultAppRoleAuthSpec specifies approle-specific auth data
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorDestinationVaultAuthApprole {
    /// approle Vault prefix. Default: approle
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appRolePath")]
    pub app_role_path: Option<String>,
    /// A key in the SecretRef which contains role-id value. Default: role-id
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleIDKey")]
    pub role_id_key: Option<String>,
    /// A key in the SecretRef which contains secret-id value. Default: secret-id
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretIDKey")]
    pub secret_id_key: Option<String>,
    /// Reference to a Secret containing role-id and secret-id
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<SecretMirrorDestinationVaultAuthApproleSecretRef>,
}

/// Reference to a Secret containing role-id and secret-id
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorDestinationVaultAuthApproleSecretRef {
    /// Name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// VaultTokenAuthSpec specifies token-specific auth data
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorDestinationVaultAuthToken {
    /// Reference to a Secret containing token
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<SecretMirrorDestinationVaultAuthTokenSecretRef>,
    /// A key in the SecretRef which contains token value. Default: token
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenKey")]
    pub token_key: Option<String>,
}

/// Reference to a Secret containing token
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorDestinationVaultAuthTokenSecretRef {
    /// Name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// SecretMirrorSource defines where to extract a secret data from
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<SecretMirrorSourceType>,
    /// VaultSpec contains information of secret location
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vault: Option<SecretMirrorSourceVault>,
}

/// SecretMirrorSource defines where to extract a secret data from
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecretMirrorSourceType {
    #[serde(rename = "secret")]
    Secret,
    #[serde(rename = "vault")]
    Vault,
}

/// VaultSpec contains information of secret location
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorSourceVault {
    /// Addr specifies a Vault endpoint URL (e.g. https://vault.example.com)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    /// VaultAuthSpec describes how to authenticate against a Vault server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<SecretMirrorSourceVaultAuth>,
    /// Path specifies a vault secret path (e.g. secret/data/some-secret or mongodb/creds/mymongo)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// VaultAuthSpec describes how to authenticate against a Vault server
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorSourceVaultAuth {
    /// VaultAppRoleAuthSpec specifies approle-specific auth data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approle: Option<SecretMirrorSourceVaultAuthApprole>,
    /// VaultTokenAuthSpec specifies token-specific auth data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<SecretMirrorSourceVaultAuthToken>,
}

/// VaultAppRoleAuthSpec specifies approle-specific auth data
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorSourceVaultAuthApprole {
    /// approle Vault prefix. Default: approle
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appRolePath")]
    pub app_role_path: Option<String>,
    /// A key in the SecretRef which contains role-id value. Default: role-id
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleIDKey")]
    pub role_id_key: Option<String>,
    /// A key in the SecretRef which contains secret-id value. Default: secret-id
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretIDKey")]
    pub secret_id_key: Option<String>,
    /// Reference to a Secret containing role-id and secret-id
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<SecretMirrorSourceVaultAuthApproleSecretRef>,
}

/// Reference to a Secret containing role-id and secret-id
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorSourceVaultAuthApproleSecretRef {
    /// Name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// VaultTokenAuthSpec specifies token-specific auth data
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorSourceVaultAuthToken {
    /// Reference to a Secret containing token
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<SecretMirrorSourceVaultAuthTokenSecretRef>,
    /// A key in the SecretRef which contains token value. Default: token
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenKey")]
    pub token_key: Option<String>,
}

/// Reference to a Secret containing token
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorSourceVaultAuthTokenSecretRef {
    /// Name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// SecretMirrorStatus defines the observed state of SecretMirror
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorStatus {
    /// Timestamp of last successful mirrorring
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSyncTime")]
    pub last_sync_time: Option<String>,
    /// Mirroring status - Active, Pending or Error
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mirrorStatus")]
    pub mirror_status: Option<SecretMirrorStatusMirrorStatus>,
    /// VaultSourceStatusSpec describes Vault-specific status
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vaultSource")]
    pub vault_source: Option<SecretMirrorStatusVaultSource>,
}

/// SecretMirrorStatus defines the observed state of SecretMirror
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecretMirrorStatusMirrorStatus {
    Pending,
    Active,
    Error,
}

/// VaultSourceStatusSpec describes Vault-specific status
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorStatusVaultSource {
    /// Contains lease duration of a Vault dynamic secret
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "leaseDuration")]
    pub lease_duration: Option<i64>,
    /// Contains LeaseID of a Vault dynamic secret
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "leaseID")]
    pub lease_id: Option<String>,
}

