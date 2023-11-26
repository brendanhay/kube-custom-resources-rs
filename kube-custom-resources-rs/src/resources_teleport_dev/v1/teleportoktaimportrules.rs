// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/gravitational/teleport/resources.teleport.dev/v1/teleportoktaimportrules.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// OktaImportRule resource definition v1 from Teleport
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "resources.teleport.dev", version = "v1", kind = "TeleportOktaImportRule", plural = "teleportoktaimportrules")]
#[kube(namespaced)]
#[kube(status = "TeleportOktaImportRuleStatus")]
#[kube(schema = "disabled")]
pub struct TeleportOktaImportRuleSpec {
    /// Mappings is a list of matches that will map match conditions to labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<TeleportOktaImportRuleMappings>>,
    /// Priority represents the priority of the rule application. Lower numbered rules will be applied first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TeleportOktaImportRuleMappings {
    /// AddLabels specifies which labels to add if any of the previous matches match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_labels: Option<TeleportOktaImportRuleMappingsAddLabels>,
    /// Match is a set of matching rules for this mapping. If any of these match, then the mapping will be applied.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<Vec<TeleportOktaImportRuleMappingsMatch>>,
}

/// AddLabels specifies which labels to add if any of the previous matches match.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TeleportOktaImportRuleMappingsAddLabels {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TeleportOktaImportRuleMappingsMatch {
    /// AppIDs is a list of app IDs to match against.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_ids: Option<Vec<String>>,
    /// AppNameRegexes is a list of regexes to match against app names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_name_regexes: Option<Vec<String>>,
    /// GroupIDs is a list of group IDs to match against.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// GroupNameRegexes is a list of regexes to match against group names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_name_regexes: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TeleportOktaImportRuleStatus {
    /// Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<TeleportOktaImportRuleStatusConditions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "teleportResourceID")]
    pub teleport_resource_id: Option<i64>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TeleportOktaImportRuleStatusConditions {
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
    pub status: TeleportOktaImportRuleStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TeleportOktaImportRuleStatusConditionsStatus {
    True,
    False,
    Unknown,
}
