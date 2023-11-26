// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/snapshots.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// SnapshotSpec defines the desired state of Longhorn Snapshot
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "Snapshot", plural = "snapshots")]
#[kube(namespaced)]
#[kube(status = "SnapshotStatus")]
#[kube(schema = "disabled")]
pub struct SnapshotSpec {
    /// require creating a new snapshot
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createSnapshot")]
    pub create_snapshot: Option<bool>,
    /// The labels of snapshot
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// the volume that this snapshot belongs to. This field is immutable after creation. Required
    pub volume: String,
}

/// SnapshotStatus defines the observed state of Longhorn Snapshot
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnapshotStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<BTreeMap<String, bool>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationTime")]
    pub creation_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "markRemoved")]
    pub mark_removed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyToUse")]
    pub ready_to_use: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restoreSize")]
    pub restore_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userCreated")]
    pub user_created: Option<bool>,
}
