// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/openshift/hive/hive.openshift.io/v1/checkpoints.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// CheckpointSpec defines the metadata around the Hive objects state in the namespace at the time of the last backup.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "hive.openshift.io", version = "v1", kind = "Checkpoint", plural = "checkpoints")]
#[kube(namespaced)]
#[kube(status = "CheckpointStatus")]
#[kube(schema = "disabled")]
pub struct CheckpointSpec {
    /// LastBackupChecksum is the checksum of all Hive objects in the namespace at the time of the last backup.
    #[serde(rename = "lastBackupChecksum")]
    pub last_backup_checksum: String,
    /// LastBackupRef is a reference to last backup object created
    #[serde(rename = "lastBackupRef")]
    pub last_backup_ref: CheckpointLastBackupRef,
    /// LastBackupTime is the last time we performed a backup of the namespace
    #[serde(rename = "lastBackupTime")]
    pub last_backup_time: String,
}

/// LastBackupRef is a reference to last backup object created
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CheckpointLastBackupRef {
    pub name: String,
    pub namespace: String,
}

/// CheckpointStatus defines the observed state of Checkpoint
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CheckpointStatus {
}

