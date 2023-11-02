// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/clusternet/clusternet/clusters.clusternet.io/v1beta1/managedclusters.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// ManagedClusterSpec defines the desired state of ManagedCluster
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "clusters.clusternet.io", version = "v1beta1", kind = "ManagedCluster", plural = "managedclusters")]
#[kube(namespaced)]
#[kube(status = "ManagedClusterStatus")]
#[kube(schema = "disabled")]
pub struct ManagedClusterSpec {
    /// ClusterID, a Random (Version 4) UUID, is a unique value in time and space value representing for child cluster. It is typically generated by the clusternet agent on the successful creation of a "clusternet-agent" Lease in the child cluster. Also it is not allowed to change on PUT operations.
    #[serde(rename = "clusterId")]
    pub cluster_id: String,
    /// ClusterType denotes the type of the child cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterType")]
    pub cluster_type: Option<String>,
    /// SyncMode decides how to sync resources from parent cluster to child cluster.
    #[serde(rename = "syncMode")]
    pub sync_mode: ManagedClusterSyncMode,
    /// Taints has the "effect" on any resource that does not tolerate the Taint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<ManagedClusterTaints>>,
}

/// ManagedClusterSpec defines the desired state of ManagedCluster
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ManagedClusterSyncMode {
    Push,
    Pull,
    Dual,
}

/// The node this Taint is attached to has the "effect" on any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ManagedClusterTaints {
    /// Required. The effect of the taint on pods that do not tolerate the taint. Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
    pub effect: String,
    /// Required. The taint key to be applied to a node.
    pub key: String,
    /// TimeAdded represents the time at which the taint was added. It is only written for NoExecute taints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeAdded")]
    pub time_added: Option<String>,
    /// The taint value corresponding to the taint key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ManagedClusterStatus defines the observed state of ManagedCluster
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ManagedClusterStatus {
    /// Allocatable is the sum of allocatable resources for nodes in the cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocatable: Option<BTreeMap<String, IntOrString>>,
    /// APIServerURL indicates the advertising url/address of managed Kubernetes cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiserverURL")]
    pub apiserver_url: Option<String>,
    /// AppPusher indicates whether to allow parent cluster deploying applications in Push or Dual Mode. Mainly for security concerns.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appPusher")]
    pub app_pusher: Option<bool>,
    /// Capacity is the sum of capacity resources for nodes in the cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<BTreeMap<String, IntOrString>>,
    /// ClusterCIDR is the CIDR range of the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterCIDR")]
    pub cluster_cidr: Option<String>,
    /// Conditions is an array of current cluster conditions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ManagedClusterStatusConditions>>,
    /// Healthz indicates the healthz status of the cluster which is deprecated since Kubernetes v1.16. Please use Livez and Readyz instead. Leave it here only for compatibility.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healthz: Option<bool>,
    /// heartbeatFrequencySeconds is the frequency at which the agent reports current cluster status
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "heartbeatFrequencySeconds")]
    pub heartbeat_frequency_seconds: Option<i64>,
    /// k8sVersion is the Kubernetes version of the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "k8sVersion")]
    pub k8s_version: Option<String>,
    /// KubeBurst allows extra queries to accumulate when a client is exceeding its rate. Used by deployer in Clusternet to control the burst to current child cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubeBurst")]
    pub kube_burst: Option<i32>,
    /// KubeQPS controls the number of queries per second allowed for this connection. Used by deployer in Clusternet to control the qps to current child cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubeQPS")]
    pub kube_qps: Option<f64>,
    /// lastObservedTime is the time when last status from the series was seen before last heartbeat. RFC 3339 date and time at which the object was acknowledged by the Clusternet Agent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastObservedTime")]
    pub last_observed_time: Option<String>,
    /// Livez indicates the livez status of the cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub livez: Option<bool>,
    /// NodeStatistics is the info summary of nodes in the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeStatistics")]
    pub node_statistics: Option<ManagedClusterStatusNodeStatistics>,
    /// platform indicates the running platform of the cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// PodStatistics is the info summary of pods in the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podStatistics")]
    pub pod_statistics: Option<ManagedClusterStatusPodStatistics>,
    /// PredictorAddress shows the predictor address
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "predictorAddress")]
    pub predictor_address: Option<String>,
    /// PredictorDirectAccess indicates whether the predictor can be accessed directly by clusternet-scheduler
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "predictorDirectAccess")]
    pub predictor_direct_access: Option<bool>,
    /// PredictorEnabled indicates whether predictor is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "predictorEnabled")]
    pub predictor_enabled: Option<bool>,
    /// Readyz indicates the readyz status of the cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readyz: Option<bool>,
    /// ResourceUsage is the cpu(m) and memory(Mi) already used in the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceUsage")]
    pub resource_usage: Option<ManagedClusterStatusResourceUsage>,
    /// ServcieCIDR is the CIDR range of the services
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceCIDR")]
    pub service_cidr: Option<String>,
    /// UseSocket indicates whether to use socket proxy when connecting to child cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useSocket")]
    pub use_socket: Option<bool>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ManagedClusterStatusConditions {
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
    pub status: ManagedClusterStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ManagedClusterStatusConditionsStatus {
    True,
    False,
    Unknown,
}

/// NodeStatistics is the info summary of nodes in the cluster
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ManagedClusterStatusNodeStatistics {
    /// LostNodes is the number of states lost nodes in the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lostNodes")]
    pub lost_nodes: Option<i32>,
    /// NotReadyNodes is the number of not ready nodes in the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notReadyNodes")]
    pub not_ready_nodes: Option<i32>,
    /// ReadyNodes is the number of ready nodes in the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyNodes")]
    pub ready_nodes: Option<i32>,
    /// UnknownNodes is the number of unknown nodes in the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unknownNodes")]
    pub unknown_nodes: Option<i32>,
}

/// PodStatistics is the info summary of pods in the cluster
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ManagedClusterStatusPodStatistics {
    /// RunningPods is the number of running pods in the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runningPods")]
    pub running_pods: Option<i32>,
    /// TotalPods is the number of all pods in the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "totalPods")]
    pub total_pods: Option<i32>,
}

/// ResourceUsage is the cpu(m) and memory(Mi) already used in the cluster
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ManagedClusterStatusResourceUsage {
    /// CpuUsage is the total cpu(m) already used in the whole cluster, k8s reserved not include
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cpuUsage")]
    pub cpu_usage: Option<IntOrString>,
    /// MemoryUsage is the total memory(Mi) already used in the whole cluster, k8s reserved not include
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memoryUsage")]
    pub memory_usage: Option<IntOrString>,
}

