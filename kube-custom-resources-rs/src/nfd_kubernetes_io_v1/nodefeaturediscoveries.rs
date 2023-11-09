// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/kubernetes-sigs/node-feature-discovery-operator/nfd.kubernetes.io/v1/nodefeaturediscoveries.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// NodeFeatureDiscoverySpec defines the desired state of NodeFeatureDiscovery
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "nfd.kubernetes.io", version = "v1", kind = "NodeFeatureDiscovery", plural = "nodefeaturediscoveries")]
#[kube(namespaced)]
#[kube(status = "NodeFeatureDiscoveryStatus")]
#[kube(schema = "disabled")]
pub struct NodeFeatureDiscoverySpec {
    /// ExtraLabelNs defines the list of of allowed extra label namespaces By default, only allow labels in the default `feature.node.kubernetes.io` label namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extraLabelNs")]
    pub extra_label_ns: Option<Vec<String>>,
    /// Instance name. Used to separate annotation namespaces for multiple parallel deployments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// LabelWhiteList defines a regular expression for filtering feature labels based on their name. Each label must match against the given reqular expression in order to be published.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelWhiteList")]
    pub label_white_list: Option<String>,
    /// OperandSpec describes configuration options for the operand
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operand: Option<NodeFeatureDiscoveryOperand>,
    /// PruneOnDelete defines whether the NFD-master prune should be enabled or not. If enabled, the Operator will deploy an NFD-Master prune job that will remove all NFD labels (and other NFD-managed assets such as annotations, extended resources and taints) from the cluster nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prunerOnDelete")]
    pub pruner_on_delete: Option<bool>,
    /// ResourceLabels defines the list of features to be advertised as extended resources instead of labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceLabels")]
    pub resource_labels: Option<Vec<String>>,
    /// Deploy the NFD-Topology-Updater NFD-Topology-Updater is a daemon responsible for examining allocated resources on a worker node to account for resources available to be allocated to new pod on a per-zone basis https://kubernetes-sigs.github.io/node-feature-discovery/master/get-started/introduction.html#nfd-topology-updater
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topologyUpdater")]
    pub topology_updater: Option<bool>,
    /// WorkerConfig describes configuration options for the NFD worker.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workerConfig")]
    pub worker_config: Option<NodeFeatureDiscoveryWorkerConfig>,
}

/// OperandSpec describes configuration options for the operand
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryOperand {
    /// Image defines the image to pull for the NFD operand [defaults to registry.k8s.io/nfd/node-feature-discovery]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// ImagePullPolicy defines Image pull policy for the NFD operand image [defaults to Always]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
    /// ServicePort specifies the TCP port that nfd-master listens for incoming requests.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "servicePort")]
    pub service_port: Option<i64>,
}

/// WorkerConfig describes configuration options for the NFD worker.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryWorkerConfig {
    /// BinaryData holds the NFD configuration file
    #[serde(rename = "configData")]
    pub config_data: String,
}

/// NodeFeatureDiscoveryStatus defines the observed state of NodeFeatureDiscovery
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryStatus {
    /// Conditions represents the latest available observations of current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<NodeFeatureDiscoveryStatusConditions>>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryStatusConditions {
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
    pub status: NodeFeatureDiscoveryStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodeFeatureDiscoveryStatusConditionsStatus {
    True,
    False,
    Unknown,
}

