// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/kubernetes-sigs/cluster-api/cluster.x-k8s.io/v1alpha4/machinehealthchecks.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// Specification of machine health check policy
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "cluster.x-k8s.io", version = "v1alpha4", kind = "MachineHealthCheck", plural = "machinehealthchecks")]
#[kube(namespaced)]
#[kube(status = "MachineHealthCheckStatus")]
#[kube(schema = "disabled")]
pub struct MachineHealthCheckSpec {
    /// ClusterName is the name of the Cluster this object belongs to.
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// Any further remediation is only allowed if at most "MaxUnhealthy" machines selected by "selector" are not healthy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUnhealthy")]
    pub max_unhealthy: Option<IntOrString>,
    /// Machines older than this duration without a node will be considered to have failed and will be remediated. If not set, this value is defaulted to 10 minutes. If you wish to disable this feature, set the value explicitly to 0.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeStartupTimeout")]
    pub node_startup_timeout: Option<String>,
    /// RemediationTemplate is a reference to a remediation template provided by an infrastructure provider. 
    ///  This field is completely optional, when filled, the MachineHealthCheck controller creates a new object from the template referenced and hands off remediation of the machine to a controller that lives outside of Cluster API.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remediationTemplate")]
    pub remediation_template: Option<MachineHealthCheckRemediationTemplate>,
    /// Label selector to match machines whose health will be exercised
    pub selector: MachineHealthCheckSelector,
    /// UnhealthyConditions contains a list of the conditions that determine whether a node is considered unhealthy.  The conditions are combined in a logical OR, i.e. if any of the conditions is met, the node is unhealthy.
    #[serde(rename = "unhealthyConditions")]
    pub unhealthy_conditions: Vec<MachineHealthCheckUnhealthyConditions>,
    /// Any further remediation is only allowed if the number of machines selected by "selector" as not healthy is within the range of "UnhealthyRange". Takes precedence over MaxUnhealthy. Eg. "[3-5]" - This means that remediation will be allowed only when: (a) there are at least 3 unhealthy machines (and) (b) there are at most 5 unhealthy machines
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unhealthyRange")]
    pub unhealthy_range: Option<String>,
}

/// RemediationTemplate is a reference to a remediation template provided by an infrastructure provider. 
///  This field is completely optional, when filled, the MachineHealthCheck controller creates a new object from the template referenced and hands off remediation of the machine to a controller that lives outside of Cluster API.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MachineHealthCheckRemediationTemplate {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object. TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// Label selector to match machines whose health will be exercised
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MachineHealthCheckSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<MachineHealthCheckSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MachineHealthCheckSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// UnhealthyCondition represents a Node condition type and value with a timeout specified as a duration.  When the named condition has been in the given status for at least the timeout value, a node is considered unhealthy.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MachineHealthCheckUnhealthyConditions {
    pub status: String,
    pub timeout: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Most recently observed status of MachineHealthCheck resource
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MachineHealthCheckStatus {
    /// Conditions defines current service state of the MachineHealthCheck.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<MachineHealthCheckStatusConditions>>,
    /// total number of healthy machines counted by this machine health check
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentHealthy")]
    pub current_healthy: Option<i32>,
    /// total number of machines counted by this machine health check
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expectedMachines")]
    pub expected_machines: Option<i32>,
    /// ObservedGeneration is the latest generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// RemediationsAllowed is the number of further remediations allowed by this machine health check before maxUnhealthy short circuiting will be applied
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remediationsAllowed")]
    pub remediations_allowed: Option<i32>,
    /// Targets shows the current list of machines the machine health check is watching
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

/// Condition defines an observation of a Cluster API resource operational state.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MachineHealthCheckStatusConditions {
    /// Last time the condition transitioned from one status to another. This should be when the underlying condition changed. If that is not known, then using the time when the API field changed is acceptable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// A human readable message indicating details about the transition. This field may be empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition in CamelCase. The specific API may choose whether or not this field is considered a guaranteed API. This field may not be empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Severity provides an explicit classification of Reason code, so the users or machines can immediately understand the current situation and act accordingly. The Severity field MUST be set only when Status=False.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// Type of condition in CamelCase or in foo.example.com/CamelCase. Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important.
    #[serde(rename = "type")]
    pub r#type: String,
}

