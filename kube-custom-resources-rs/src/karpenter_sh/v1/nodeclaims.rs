// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws/karpenter-provider-aws/karpenter.sh/v1/nodeclaims.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// NodeClaimSpec describes the desired state of the NodeClaim
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "karpenter.sh", version = "v1", kind = "NodeClaim", plural = "nodeclaims")]
#[kube(status = "NodeClaimStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct NodeClaimSpec {
    /// Kubelet defines args to be used when configuring kubelet on provisioned nodes.
    /// They are a subset of the upstream types, recognizing not all options may be supported.
    /// Wherever possible, the types and names should reflect the upstream kubelet types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubelet: Option<NodeClaimKubelet>,
    /// NodeClassRef is a reference to an object that defines provider specific configuration
    #[serde(rename = "nodeClassRef")]
    pub node_class_ref: NodeClaimNodeClassRef,
    /// Requirements are layered with GetLabels and applied to every node.
    pub requirements: Vec<NodeClaimRequirements>,
    /// Resources models the resource requirements for the NodeClaim to launch
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<NodeClaimResources>,
    /// StartupTaints are taints that are applied to nodes upon startup which are expected to be removed automatically
    /// within a short period of time, typically by a DaemonSet that tolerates the taint. These are commonly used by
    /// daemonsets to allow initialization and enforce startup ordering.  StartupTaints are ignored for provisioning
    /// purposes in that pods are not required to tolerate a StartupTaint in order to have nodes provisioned for them.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startupTaints")]
    pub startup_taints: Option<Vec<NodeClaimStartupTaints>>,
    /// Taints will be applied to the NodeClaim's node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<NodeClaimTaints>>,
}

/// Kubelet defines args to be used when configuring kubelet on provisioned nodes.
/// They are a subset of the upstream types, recognizing not all options may be supported.
/// Wherever possible, the types and names should reflect the upstream kubelet types.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeClaimKubelet {
    /// clusterDNS is a list of IP addresses for the cluster DNS server.
    /// Note that not all providers may use all addresses.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterDNS")]
    pub cluster_dns: Option<Vec<String>>,
    /// CPUCFSQuota enables CPU CFS quota enforcement for containers that specify CPU limits.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cpuCFSQuota")]
    pub cpu_cfs_quota: Option<bool>,
    /// EvictionHard is the map of signal names to quantities that define hard eviction thresholds
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionHard")]
    pub eviction_hard: Option<BTreeMap<String, String>>,
    /// EvictionMaxPodGracePeriod is the maximum allowed grace period (in seconds) to use when terminating pods in
    /// response to soft eviction thresholds being met.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionMaxPodGracePeriod")]
    pub eviction_max_pod_grace_period: Option<i32>,
    /// EvictionSoft is the map of signal names to quantities that define soft eviction thresholds
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionSoft")]
    pub eviction_soft: Option<BTreeMap<String, String>>,
    /// EvictionSoftGracePeriod is the map of signal names to quantities that define grace periods for each eviction signal
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionSoftGracePeriod")]
    pub eviction_soft_grace_period: Option<BTreeMap<String, String>>,
    /// ImageGCHighThresholdPercent is the percent of disk usage after which image
    /// garbage collection is always run. The percent is calculated by dividing this
    /// field value by 100, so this field must be between 0 and 100, inclusive.
    /// When specified, the value must be greater than ImageGCLowThresholdPercent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageGCHighThresholdPercent")]
    pub image_gc_high_threshold_percent: Option<i32>,
    /// ImageGCLowThresholdPercent is the percent of disk usage before which image
    /// garbage collection is never run. Lowest disk usage to garbage collect to.
    /// The percent is calculated by dividing this field value by 100,
    /// so the field value must be between 0 and 100, inclusive.
    /// When specified, the value must be less than imageGCHighThresholdPercent
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageGCLowThresholdPercent")]
    pub image_gc_low_threshold_percent: Option<i32>,
    /// KubeReserved contains resources reserved for Kubernetes system components.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubeReserved")]
    pub kube_reserved: Option<BTreeMap<String, String>>,
    /// MaxPods is an override for the maximum number of pods that can run on
    /// a worker node instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxPods")]
    pub max_pods: Option<i32>,
    /// PodsPerCore is an override for the number of pods that can run on a worker node
    /// instance based on the number of cpu cores. This value cannot exceed MaxPods, so, if
    /// MaxPods is a lower value, that value will be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podsPerCore")]
    pub pods_per_core: Option<i32>,
    /// SystemReserved contains resources reserved for OS system daemons and kernel memory.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "systemReserved")]
    pub system_reserved: Option<BTreeMap<String, String>>,
}

/// NodeClassRef is a reference to an object that defines provider specific configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeClaimNodeClassRef {
    /// API version of the referent
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent; More info: http://kubernetes.io/docs/user-guide/identifiers#names
    pub name: String,
}

/// A node selector requirement with min values is a selector that contains values, a key, an operator that relates the key and values
/// and minValues that represent the requirement to have at least that many values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NodeClaimRequirements {
    /// The label key that the selector applies to.
    pub key: String,
    /// This field is ALPHA and can be dropped or replaced at any time
    /// MinValues is the minimum number of unique values required to define the flexibility of the specific requirement.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minValues")]
    pub min_values: Option<i64>,
    /// Represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: NodeClaimRequirementsOperator,
    /// An array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. If the operator is Gt or Lt, the values
    /// array must have a single element, which will be interpreted as an integer.
    /// This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A node selector requirement with min values is a selector that contains values, a key, an operator that relates the key and values
/// and minValues that represent the requirement to have at least that many values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodeClaimRequirementsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
    Gt,
    Lt,
}

/// Resources models the resource requirements for the NodeClaim to launch
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeClaimResources {
    /// Requests describes the minimum required resources for the NodeClaim to launch
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// The node this Taint is attached to has the "effect" on
/// any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NodeClaimStartupTaints {
    /// Required. The effect of the taint on pods
    /// that do not tolerate the taint.
    /// Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
    pub effect: NodeClaimStartupTaintsEffect,
    /// Required. The taint key to be applied to a node.
    pub key: String,
    /// TimeAdded represents the time at which the taint was added.
    /// It is only written for NoExecute taints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeAdded")]
    pub time_added: Option<String>,
    /// The taint value corresponding to the taint key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// The node this Taint is attached to has the "effect" on
/// any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodeClaimStartupTaintsEffect {
    NoSchedule,
    PreferNoSchedule,
    NoExecute,
}

/// The node this Taint is attached to has the "effect" on
/// any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NodeClaimTaints {
    /// Required. The effect of the taint on pods
    /// that do not tolerate the taint.
    /// Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
    pub effect: NodeClaimTaintsEffect,
    /// Required. The taint key to be applied to a node.
    pub key: String,
    /// TimeAdded represents the time at which the taint was added.
    /// It is only written for NoExecute taints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeAdded")]
    pub time_added: Option<String>,
    /// The taint value corresponding to the taint key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// The node this Taint is attached to has the "effect" on
/// any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodeClaimTaintsEffect {
    NoSchedule,
    PreferNoSchedule,
    NoExecute,
}

/// NodeClaimStatus defines the observed state of NodeClaim
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeClaimStatus {
    /// Allocatable is the estimated allocatable capacity of the node
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocatable: Option<BTreeMap<String, IntOrString>>,
    /// Capacity is the estimated full capacity of the node
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<BTreeMap<String, IntOrString>>,
    /// Conditions contains signals for health and readiness
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ImageID is an identifier for the image that runs on the node
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageID")]
    pub image_id: Option<String>,
    /// NodeName is the name of the corresponding node object
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeName")]
    pub node_name: Option<String>,
    /// ProviderID of the corresponding node object
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerID")]
    pub provider_id: Option<String>,
}

