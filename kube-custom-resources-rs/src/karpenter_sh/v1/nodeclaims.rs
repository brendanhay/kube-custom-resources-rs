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
    /// ExpireAfter is the duration the controller will wait
    /// before terminating a node, measured from when the node is created. This
    /// is useful to implement features like eventually consistent node upgrade,
    /// memory leak protection, and disruption testing.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expireAfter")]
    pub expire_after: Option<String>,
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
    /// TerminationGracePeriod is the maximum duration the controller will wait before forcefully deleting the pods on a node, measured from when deletion is first initiated.
    /// 
    /// Warning: this feature takes precedence over a Pod's terminationGracePeriodSeconds value, and bypasses any blocked PDBs or the karpenter.sh/do-not-disrupt annotation.
    /// 
    /// This field is intended to be used by cluster administrators to enforce that nodes can be cycled within a given time period.
    /// When set, drifted nodes will begin draining even if there are pods blocking eviction. Draining will respect PDBs and the do-not-disrupt annotation until the TGP is reached.
    /// 
    /// Karpenter will preemptively delete pods so their terminationGracePeriodSeconds align with the node's terminationGracePeriod.
    /// If a pod would be terminated without being granted its full terminationGracePeriodSeconds prior to the node timeout,
    /// that pod will be deleted at T = node timeout - pod terminationGracePeriodSeconds.
    /// 
    /// The feature can also be used to allow maximum time limits for long-running jobs which can delay node termination with preStop hooks.
    /// If left undefined, the controller will wait indefinitely for pods to be drained.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "terminationGracePeriod")]
    pub termination_grace_period: Option<String>,
}

/// NodeClassRef is a reference to an object that defines provider specific configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeClaimNodeClassRef {
    /// API version of the referent
    pub group: String,
    /// Kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"
    pub kind: String,
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
    /// LastPodEventTime is updated with the last time a pod was scheduled
    /// or removed from the node. A pod going terminal or terminating
    /// is also considered as removed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastPodEventTime")]
    pub last_pod_event_time: Option<String>,
    /// NodeName is the name of the corresponding node object
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeName")]
    pub node_name: Option<String>,
    /// ProviderID of the corresponding node object
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerID")]
    pub provider_id: Option<String>,
}

