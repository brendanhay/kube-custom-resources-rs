// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/orphans.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// OrphanSpec defines the desired state of the Longhorn orphaned data
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "Orphan", plural = "orphans")]
#[kube(namespaced)]
#[kube(status = "OrphanStatus")]
#[kube(schema = "disabled")]
pub struct OrphanSpec {
    /// The node ID on which the controller is responsible to reconcile this orphan CR.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    pub node_id: Option<String>,
    /// The type of the orphaned data. Can be "replica".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "orphanType")]
    pub orphan_type: Option<String>,
    /// The parameters of the orphaned data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, String>>,
}

/// OrphanStatus defines the observed state of the Longhorn orphaned data
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrphanStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<OrphanStatusConditions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrphanStatusConditions {
    /// Last time we probed the condition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastProbeTime")]
    pub last_probe_time: Option<String>,
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status is the status of the condition. Can be True, False, Unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Type is the type of the condition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}
