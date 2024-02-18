// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/cluster-api/cluster.x-k8s.io/v1alpha3/clusters.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// ClusterSpec defines the desired state of Cluster.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cluster.x-k8s.io", version = "v1alpha3", kind = "Cluster", plural = "clusters")]
#[kube(namespaced)]
#[kube(status = "ClusterStatus")]
#[kube(schema = "disabled")]
pub struct ClusterSpec {
    /// Cluster network configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterNetwork")]
    pub cluster_network: Option<ClusterClusterNetwork>,
    /// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneEndpoint")]
    pub control_plane_endpoint: Option<ClusterControlPlaneEndpoint>,
    /// ControlPlaneRef is an optional reference to a provider-specific resource that holds
    /// the details for provisioning the Control Plane for a Cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneRef")]
    pub control_plane_ref: Option<ClusterControlPlaneRef>,
    /// InfrastructureRef is a reference to a provider-specific resource that holds the details
    /// for provisioning infrastructure for a cluster in said provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "infrastructureRef")]
    pub infrastructure_ref: Option<ClusterInfrastructureRef>,
    /// Paused can be used to prevent controllers from processing the Cluster and all its associated objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
}

/// Cluster network configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClusterNetwork {
    /// APIServerPort specifies the port the API Server should bind to.
    /// Defaults to 6443.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiServerPort")]
    pub api_server_port: Option<i32>,
    /// The network ranges from which Pod networks are allocated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<ClusterClusterNetworkPods>,
    /// Domain name for services.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceDomain")]
    pub service_domain: Option<String>,
    /// The network ranges from which service VIPs are allocated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<ClusterClusterNetworkServices>,
}

/// The network ranges from which Pod networks are allocated.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClusterNetworkPods {
    #[serde(rename = "cidrBlocks")]
    pub cidr_blocks: Vec<String>,
}

/// The network ranges from which service VIPs are allocated.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClusterNetworkServices {
    #[serde(rename = "cidrBlocks")]
    pub cidr_blocks: Vec<String>,
}

/// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterControlPlaneEndpoint {
    /// The hostname on which the API server is serving.
    pub host: String,
    /// The port on which the API server is serving.
    pub port: i32,
}

/// ControlPlaneRef is an optional reference to a provider-specific resource that holds
/// the details for provisioning the Control Plane for a Cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterControlPlaneRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    /// TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// InfrastructureRef is a reference to a provider-specific resource that holds the details
/// for provisioning infrastructure for a cluster in said provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterInfrastructureRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    /// TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// ClusterStatus defines the observed state of Cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterStatus {
    /// Conditions defines current service state of the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ClusterStatusConditions>>,
    /// ControlPlaneInitialized defines if the control plane has been initialized.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneInitialized")]
    pub control_plane_initialized: Option<bool>,
    /// ControlPlaneReady defines if the control plane is ready.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneReady")]
    pub control_plane_ready: Option<bool>,
    /// FailureDomains is a slice of failure domain objects synced from the infrastructure provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureDomains")]
    pub failure_domains: Option<BTreeMap<String, ClusterStatusFailureDomains>>,
    /// FailureMessage indicates that there is a fatal problem reconciling the
    /// state, and will be set to a descriptive error message.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureMessage")]
    pub failure_message: Option<String>,
    /// FailureReason indicates that there is a fatal problem reconciling the
    /// state, and will be set to a token value suitable for
    /// programmatic interpretation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// InfrastructureReady is the state of the infrastructure provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "infrastructureReady")]
    pub infrastructure_ready: Option<bool>,
    /// ObservedGeneration is the latest generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Phase represents the current phase of cluster actuation.
    /// E.g. Pending, Running, Terminating, Failed etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

/// Condition defines an observation of a Cluster API resource operational state.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterStatusConditions {
    /// Last time the condition transitioned from one status to another.
    /// This should be when the underlying condition changed. If that is not known, then using the time when
    /// the API field changed is acceptable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// A human readable message indicating details about the transition.
    /// This field may be empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition in CamelCase.
    /// The specific API may choose whether or not this field is considered a guaranteed API.
    /// This field may not be empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Severity provides an explicit classification of Reason code, so the users or machines can immediately
    /// understand the current situation and act accordingly.
    /// The Severity field MUST be set only when Status=False.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// Type of condition in CamelCase or in foo.example.com/CamelCase.
    /// Many .condition.type values are consistent across resources like Available, but because arbitrary conditions
    /// can be useful (see .node.status.conditions), the ability to deconflict is important.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// FailureDomains is a slice of failure domain objects synced from the infrastructure provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterStatusFailureDomains {
    /// Attributes is a free form map of attributes an infrastructure provider might use or require.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<BTreeMap<String, String>>,
    /// ControlPlane determines if this failure domain is suitable for use by control plane machines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlane")]
    pub control_plane: Option<bool>,
}
