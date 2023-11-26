// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/gravitational/teleport/resources.teleport.dev/v3/teleportoidcconnectors.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// OIDCConnector resource definition v3 from Teleport
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "resources.teleport.dev", version = "v3", kind = "TeleportOIDCConnector", plural = "teleportoidcconnectors")]
#[kube(namespaced)]
#[kube(status = "TeleportOIDCConnectorStatus")]
#[kube(schema = "disabled")]
pub struct TeleportOIDCConnectorSpec {
    /// ACR is an Authentication Context Class Reference value. The meaning of the ACR value is context-specific and varies for identity providers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acr_values: Option<String>,
    /// AllowUnverifiedEmail tells the connector to accept OIDC users with unverified emails.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_unverified_email: Option<bool>,
    /// ClaimsToRoles specifies a dynamic mapping from claims to roles.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims_to_roles: Option<Vec<TeleportOIDCConnectorClaimsToRoles>>,
    /// ClientID is the id of the authentication client (Teleport Auth server).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// ClientSecret is used to authenticate the client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// Display is the friendly name for this provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    /// GoogleAdminEmail is the email of a google admin to impersonate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_admin_email: Option<String>,
    /// GoogleServiceAccount is a string containing google service account credentials.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_service_account: Option<String>,
    /// GoogleServiceAccountURI is a path to a google service account uri.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_service_account_uri: Option<String>,
    /// IssuerURL is the endpoint of the provider, e.g. https://accounts.google.com.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer_url: Option<String>,
    /// MaxAge is the amount of time that user logins are valid for. If a user logs in, but then does not login again within this time period, they will be forced to re-authenticate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<String>,
    /// Prompt is an optional OIDC prompt. An empty string omits prompt. If not specified, it defaults to select_account for backwards compatibility.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Provider is the external identity provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// RedirectURLs is a list of callback URLs which the identity provider can use to redirect the client back to the Teleport Proxy to complete authentication. This list should match the URLs on the provider's side. The URL used for a given auth request will be chosen to match the requesting Proxy's public address. If there is no match, the first url in the list will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<Vec<String>>,
    /// Scope specifies additional scopes set by provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    /// UsernameClaim specifies the name of the claim from the OIDC connector to be used as the user's username.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username_claim: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TeleportOIDCConnectorClaimsToRoles {
    /// Claim is a claim name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim: Option<String>,
    /// Roles is a list of static teleport roles to match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /// Value is a claim value to match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// TeleportOIDCConnectorStatus defines the observed state of TeleportOIDCConnector
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TeleportOIDCConnectorStatus {
    /// Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<TeleportOIDCConnectorStatusConditions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "teleportResourceID")]
    pub teleport_resource_id: Option<i64>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TeleportOIDCConnectorStatusConditions {
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
    pub status: TeleportOIDCConnectorStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TeleportOIDCConnectorStatusConditionsStatus {
    True,
    False,
    Unknown,
}
