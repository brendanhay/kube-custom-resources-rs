// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/argoproj-labs/argocd-operator/argoproj.io/v1alpha1/appprojects.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// AppProjectSpec is the specification of an AppProject
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "argoproj.io", version = "v1alpha1", kind = "AppProject", plural = "appprojects")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct AppProjectSpec {
    /// ClusterResourceBlacklist contains list of blacklisted cluster level resources
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterResourceBlacklist")]
    pub cluster_resource_blacklist: Option<Vec<AppProjectClusterResourceBlacklist>>,
    /// ClusterResourceWhitelist contains list of whitelisted cluster level resources
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterResourceWhitelist")]
    pub cluster_resource_whitelist: Option<Vec<AppProjectClusterResourceWhitelist>>,
    /// Description contains optional project description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Destinations contains list of destinations available for deployment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<AppProjectDestinations>>,
    /// NamespaceResourceBlacklist contains list of blacklisted namespace level resources
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceResourceBlacklist")]
    pub namespace_resource_blacklist: Option<Vec<AppProjectNamespaceResourceBlacklist>>,
    /// NamespaceResourceWhitelist contains list of whitelisted namespace level resources
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceResourceWhitelist")]
    pub namespace_resource_whitelist: Option<Vec<AppProjectNamespaceResourceWhitelist>>,
    /// OrphanedResources specifies if controller should monitor orphaned resources of apps in this project
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "orphanedResources")]
    pub orphaned_resources: Option<AppProjectOrphanedResources>,
    /// PermitOnlyProjectScopedClusters determines whether destinations can only reference clusters which are project-scoped
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "permitOnlyProjectScopedClusters")]
    pub permit_only_project_scoped_clusters: Option<bool>,
    /// Roles are user defined RBAC roles associated with this project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<AppProjectRoles>>,
    /// SignatureKeys contains a list of PGP key IDs that commits in Git must be signed with in order to be allowed for sync
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signatureKeys")]
    pub signature_keys: Option<Vec<AppProjectSignatureKeys>>,
    /// SourceNamespaces defines the namespaces application resources are allowed to be created in
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceNamespaces")]
    pub source_namespaces: Option<Vec<String>>,
    /// SourceRepos contains list of repository URLs which can be used for deployment
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceRepos")]
    pub source_repos: Option<Vec<String>>,
    /// SyncWindows controls when syncs can be run for apps in this project
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncWindows")]
    pub sync_windows: Option<Vec<AppProjectSyncWindows>>,
}

/// GroupKind specifies a Group and a Kind, but does not force a version.  This is useful for identifying concepts during lookup stages without having partially valid types
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppProjectClusterResourceBlacklist {
    pub group: String,
    pub kind: String,
}

/// GroupKind specifies a Group and a Kind, but does not force a version.  This is useful for identifying concepts during lookup stages without having partially valid types
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppProjectClusterResourceWhitelist {
    pub group: String,
    pub kind: String,
}

/// ApplicationDestination holds information about the application's destination
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppProjectDestinations {
    /// Name is an alternate way of specifying the target cluster by its symbolic name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace specifies the target namespace for the application's resources. The namespace will only be set for namespace-scoped resources that have not set a value for .metadata.namespace
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Server specifies the URL of the target cluster and must be set to the Kubernetes control plane API
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}

/// GroupKind specifies a Group and a Kind, but does not force a version.  This is useful for identifying concepts during lookup stages without having partially valid types
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppProjectNamespaceResourceBlacklist {
    pub group: String,
    pub kind: String,
}

/// GroupKind specifies a Group and a Kind, but does not force a version.  This is useful for identifying concepts during lookup stages without having partially valid types
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppProjectNamespaceResourceWhitelist {
    pub group: String,
    pub kind: String,
}

/// OrphanedResources specifies if controller should monitor orphaned resources of apps in this project
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppProjectOrphanedResources {
    /// Ignore contains a list of resources that are to be excluded from orphaned resources monitoring
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore: Option<Vec<AppProjectOrphanedResourcesIgnore>>,
    /// Warn indicates if warning condition should be created for apps which have orphaned resources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warn: Option<bool>,
}

/// OrphanedResourceKey is a reference to a resource to be ignored from
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppProjectOrphanedResourcesIgnore {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ProjectRole represents a role that has access to a project
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppProjectRoles {
    /// Description is a description of the role
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Groups are a list of OIDC group claims bound to this role
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// JWTTokens are a list of generated JWT tokens bound to this role
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwtTokens")]
    pub jwt_tokens: Option<Vec<AppProjectRolesJwtTokens>>,
    /// Name is a name for this role
    pub name: String,
    /// Policies Stores a list of casbin formatted strings that define access policies for the role in the project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
}

/// JWTToken holds the issuedAt and expiresAt values of a token
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppProjectRolesJwtTokens {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    pub iat: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// SignatureKey is the specification of a key required to verify commit signatures with
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppProjectSignatureKeys {
    /// The ID of the key in hexadecimal notation
    #[serde(rename = "keyID")]
    pub key_id: String,
}

/// SyncWindow contains the kind, time, duration and attributes that are used to assign the syncWindows to apps
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppProjectSyncWindows {
    /// Applications contains a list of applications that the window will apply to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<String>>,
    /// Clusters contains a list of clusters that the window will apply to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<String>>,
    /// Duration is the amount of time the sync window will be open
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Kind defines if the window allows or blocks syncs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// ManualSync enables manual syncs when they would otherwise be blocked
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "manualSync")]
    pub manual_sync: Option<bool>,
    /// Namespaces contains a list of namespaces that the window will apply to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// Schedule is the time the window will begin, specified in cron format
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// TimeZone of the sync that will be applied to the schedule
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeZone")]
    pub time_zone: Option<String>,
}

/// AppProjectStatus contains status information for AppProject CRs
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppProjectStatus {
    /// JWTTokensByRole contains a list of JWT tokens issued for a given role
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwtTokensByRole")]
    pub jwt_tokens_by_role: Option<BTreeMap<String, AppProjectStatusJwtTokensByRole>>,
}

/// JWTTokensByRole contains a list of JWT tokens issued for a given role
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppProjectStatusJwtTokensByRole {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AppProjectStatusJwtTokensByRoleItems>>,
}

/// JWTToken holds the issuedAt and expiresAt values of a token
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppProjectStatusJwtTokensByRoleItems {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    pub iat: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

