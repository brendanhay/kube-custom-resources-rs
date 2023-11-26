// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/example.openshift.io/v1/stableconfigtypes.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// spec is the specification of the desired behavior of the StableConfigType.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "example.openshift.io", version = "v1", kind = "StableConfigType", plural = "stableconfigtypes")]
#[kube(status = "StableConfigTypeStatus")]
#[kube(schema = "disabled")]
pub struct StableConfigTypeSpec {
    /// celUnion demonstrates how to validate a discrminated union using CEL
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "celUnion")]
    pub cel_union: Option<StableConfigTypeCelUnion>,
    /// evolvingUnion demonstrates how to phase in new values into discriminated union
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evolvingUnion")]
    pub evolving_union: Option<StableConfigTypeEvolvingUnion>,
    /// immutableField is a field that is immutable once the object has been created. It is required at all times.
    #[serde(rename = "immutableField")]
    pub immutable_field: String,
    /// optionalImmutableField is a field that is immutable once set. It is optional but may not be changed once set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "optionalImmutableField")]
    pub optional_immutable_field: Option<String>,
    /// stableField is a field that is present on default clusters and on tech preview clusters 
    ///  If empty, the platform will choose a good default, which may change over time without notice.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stableField")]
    pub stable_field: Option<String>,
}

/// celUnion demonstrates how to validate a discrminated union using CEL
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StableConfigTypeCelUnion {
    /// optionalMember is a union member that is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "optionalMember")]
    pub optional_member: Option<String>,
    /// requiredMember is a union member that is required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredMember")]
    pub required_member: Option<String>,
    /// type determines which of the union members should be populated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<StableConfigTypeCelUnionType>,
}

/// celUnion demonstrates how to validate a discrminated union using CEL
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StableConfigTypeCelUnionType {
    RequiredMember,
    OptionalMember,
    EmptyMember,
}

/// evolvingUnion demonstrates how to phase in new values into discriminated union
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StableConfigTypeEvolvingUnion {
    /// type is the discriminator. It has different values for Default and for TechPreviewNoUpgrade
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<StableConfigTypeEvolvingUnionType>,
}

/// evolvingUnion demonstrates how to phase in new values into discriminated union
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StableConfigTypeEvolvingUnionType {
    #[serde(rename = "")]
    KopiumEmpty,
    StableValue,
}

/// status is the most recently observed status of the StableConfigType.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StableConfigTypeStatus {
    /// Represents the observations of a foo's current state. Known .status.conditions.type are: "Available", "Progressing", and "Degraded"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<StableConfigTypeStatusConditions>>,
    /// immutableField is a field that is immutable once the object has been created. It is required at all times.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "immutableField")]
    pub immutable_field: Option<String>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StableConfigTypeStatusConditions {
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
    pub status: StableConfigTypeStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StableConfigTypeStatusConditionsStatus {
    True,
    False,
    Unknown,
}
