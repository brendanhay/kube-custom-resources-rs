// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/kubernetes-sigs/kueue/kueue.x-k8s.io/v1beta1/localqueues.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// LocalQueueSpec defines the desired state of LocalQueue
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "kueue.x-k8s.io", version = "v1beta1", kind = "LocalQueue", plural = "localqueues")]
#[kube(namespaced)]
#[kube(status = "LocalQueueStatus")]
pub struct LocalQueueSpec {
    /// clusterQueue is a reference to a clusterQueue that backs this localQueue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterQueue")]
    pub cluster_queue: Option<String>,
}

/// LocalQueueStatus defines the observed state of LocalQueue
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct LocalQueueStatus {
    /// admittedWorkloads is the number of workloads in this LocalQueue admitted to a ClusterQueue and that haven't finished yet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "admittedWorkloads")]
    pub admitted_workloads: Option<i32>,
    /// Conditions hold the latest available observations of the LocalQueue current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<LocalQueueStatusConditions>>,
    /// flavorsUsage are the used quotas, by flavor currently in use by the workloads assigned to this LocalQueue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flavorUsage")]
    pub flavor_usage: Option<Vec<LocalQueueStatusFlavorUsage>>,
    /// flavorsReservation are the reserved quotas, by flavor currently in use by the workloads assigned to this LocalQueue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flavorsReservation")]
    pub flavors_reservation: Option<Vec<LocalQueueStatusFlavorsReservation>>,
    /// PendingWorkloads is the number of Workloads in the LocalQueue not yet admitted to a ClusterQueue
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pendingWorkloads")]
    pub pending_workloads: Option<i32>,
    /// reservingWorkloads is the number of workloads in this LocalQueue reserving quota in a ClusterQueue and that haven't finished yet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reservingWorkloads")]
    pub reserving_workloads: Option<i32>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct LocalQueueStatusConditions {
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
    pub status: LocalQueueStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum LocalQueueStatusConditionsStatus {
    True,
    False,
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct LocalQueueStatusFlavorUsage {
    /// name of the flavor.
    pub name: String,
    /// resources lists the quota usage for the resources in this flavor.
    pub resources: Vec<LocalQueueStatusFlavorUsageResources>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct LocalQueueStatusFlavorUsageResources {
    /// name of the resource.
    pub name: String,
    /// total is the total quantity of used quota.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<IntOrString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct LocalQueueStatusFlavorsReservation {
    /// name of the flavor.
    pub name: String,
    /// resources lists the quota usage for the resources in this flavor.
    pub resources: Vec<LocalQueueStatusFlavorsReservationResources>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct LocalQueueStatusFlavorsReservationResources {
    /// name of the resource.
    pub name: String,
    /// total is the total quantity of used quota.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<IntOrString>,
}

