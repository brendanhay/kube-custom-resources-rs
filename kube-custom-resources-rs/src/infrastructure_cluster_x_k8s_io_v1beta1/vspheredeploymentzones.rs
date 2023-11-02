// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/kubernetes-sigs/cluster-api-provider-vsphere/infrastructure.cluster.x-k8s.io/v1beta1/vspheredeploymentzones.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// VSphereDeploymentZoneSpec defines the desired state of VSphereDeploymentZone.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1beta1", kind = "VSphereDeploymentZone", plural = "vspheredeploymentzones")]
#[kube(status = "VSphereDeploymentZoneStatus")]
#[kube(schema = "disabled")]
pub struct VSphereDeploymentZoneSpec {
    /// ControlPlane determines if this failure domain is suitable for use by control plane machines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlane")]
    pub control_plane: Option<bool>,
    /// FailureDomain is the name of the VSphereFailureDomain used for this VSphereDeploymentZone
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureDomain")]
    pub failure_domain: Option<String>,
    /// PlacementConstraint encapsulates the placement constraints used within this deployment zone.
    #[serde(rename = "placementConstraint")]
    pub placement_constraint: VSphereDeploymentZonePlacementConstraint,
    /// Server is the address of the vSphere endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}

/// PlacementConstraint encapsulates the placement constraints used within this deployment zone.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VSphereDeploymentZonePlacementConstraint {
    /// Folder is the name or inventory path of the folder in which the virtual machine is created/located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    /// ResourcePool is the name or inventory path of the resource pool in which the virtual machine is created/located.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourcePool")]
    pub resource_pool: Option<String>,
}

/// VSphereDeploymentZoneStatus contains the status for a VSphereDeploymentZone.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VSphereDeploymentZoneStatus {
    /// Conditions defines current service state of the VSphereMachine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<VSphereDeploymentZoneStatusConditions>>,
    /// Ready is true when the VSphereDeploymentZone resource is ready. If set to false, it will be ignored by VSphereClusters
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
}

/// Condition defines an observation of a Cluster API resource operational state.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VSphereDeploymentZoneStatusConditions {
    /// Last time the condition transitioned from one status to another. This should be when the underlying condition changed. If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
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

