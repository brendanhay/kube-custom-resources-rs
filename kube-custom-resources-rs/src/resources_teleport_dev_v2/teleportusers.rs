// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/gravitational/teleport/resources.teleport.dev/v2/teleportusers.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// User resource definition v2 from Teleport
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "resources.teleport.dev", version = "v2", kind = "TeleportUser", plural = "teleportusers")]
#[kube(namespaced)]
#[kube(status = "TeleportUserStatus")]
#[kube(schema = "disabled")]
pub struct TeleportUserSpec {
    /// GithubIdentities list associated Github OAuth2 identities that let user log in using externally verified identity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub github_identities: Option<Vec<TeleportUserGithubIdentities>>,
    /// OIDCIdentities lists associated OpenID Connect identities that let user log in using externally verified identity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oidc_identities: Option<Vec<TeleportUserOidcIdentities>>,
    /// Roles is a list of roles assigned to user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /// SAMLIdentities lists associated SAML identities that let user log in using externally verified identity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saml_identities: Option<Vec<TeleportUserSamlIdentities>>,
    /// Traits are key/value pairs received from an identity provider (through OIDC claims or SAML assertions) or from a system administrator for local accounts. Traits are used to populate role variables.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traits: Option<BTreeMap<String, String>>,
    /// TrustedDeviceIDs contains the IDs of trusted devices enrolled by the user. Managed by the Device Trust subsystem, avoid manual edits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trusted_device_ids: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TeleportUserGithubIdentities {
    /// ConnectorID is id of registered OIDC connector, e.g. 'google-example.com'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    /// Username is username supplied by external identity provider
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TeleportUserOidcIdentities {
    /// ConnectorID is id of registered OIDC connector, e.g. 'google-example.com'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    /// Username is username supplied by external identity provider
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TeleportUserSamlIdentities {
    /// ConnectorID is id of registered OIDC connector, e.g. 'google-example.com'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    /// Username is username supplied by external identity provider
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// TeleportUserStatus defines the observed state of TeleportUser
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TeleportUserStatus {
    /// Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<TeleportUserStatusConditions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "teleportResourceID")]
    pub teleport_resource_id: Option<i64>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TeleportUserStatusConditions {
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
    pub status: TeleportUserStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TeleportUserStatusConditionsStatus {
    True,
    False,
    Unknown,
}

