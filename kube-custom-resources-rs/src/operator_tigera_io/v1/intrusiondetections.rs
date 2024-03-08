// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/tigera/operator/operator.tigera.io/v1/intrusiondetections.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// Specification of the desired state for Tigera intrusion detection.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "operator.tigera.io", version = "v1", kind = "IntrusionDetection", plural = "intrusiondetections")]
#[kube(status = "IntrusionDetectionStatus")]
#[kube(schema = "disabled")]
pub struct IntrusionDetectionSpec {
    /// AnomalyDetection is now deprecated, and configuring it has no effect.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "anomalyDetection")]
    pub anomaly_detection: Option<IntrusionDetectionAnomalyDetection>,
    /// ComponentResources can be used to customize the resource requirements for each component. Only DeepPacketInspection is supported for this spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "componentResources")]
    pub component_resources: Option<Vec<IntrusionDetectionComponentResources>>,
}

/// AnomalyDetection is now deprecated, and configuring it has no effect.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IntrusionDetectionAnomalyDetection {
    /// StorageClassName is now deprecated, and configuring it has no effect.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClassName")]
    pub storage_class_name: Option<String>,
}

/// The ComponentResource struct associates a ResourceRequirements with a component by name
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IntrusionDetectionComponentResources {
    /// ComponentName is an enum which identifies the component
    #[serde(rename = "componentName")]
    pub component_name: IntrusionDetectionComponentResourcesComponentName,
    /// ResourceRequirements allows customization of limits and requests for compute resources such as cpu and memory.
    #[serde(rename = "resourceRequirements")]
    pub resource_requirements: IntrusionDetectionComponentResourcesResourceRequirements,
}

/// The ComponentResource struct associates a ResourceRequirements with a component by name
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IntrusionDetectionComponentResourcesComponentName {
    DeepPacketInspection,
}

/// ResourceRequirements allows customization of limits and requests for compute resources such as cpu and memory.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IntrusionDetectionComponentResourcesResourceRequirements {
    /// Claims lists the names of resources, defined in spec.resourceClaims, that are used by this container. 
    ///  This is an alpha field and requires enabling the DynamicResourceAllocation feature gate. 
    ///  This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<IntrusionDetectionComponentResourcesResourceRequirementsClaims>>,
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IntrusionDetectionComponentResourcesResourceRequirementsClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of the Pod where this field is used. It makes that resource available inside a container.
    pub name: String,
}

/// Most recently observed state for Tigera intrusion detection.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IntrusionDetectionStatus {
    /// Conditions represents the latest observed set of conditions for the component. A component may be one or more of Ready, Progressing, Degraded or other customer types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<IntrusionDetectionStatusConditions>>,
    /// State provides user-readable status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IntrusionDetectionStatusConditions {
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
    pub status: IntrusionDetectionStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IntrusionDetectionStatusConditionsStatus {
    True,
    False,
    Unknown,
}
