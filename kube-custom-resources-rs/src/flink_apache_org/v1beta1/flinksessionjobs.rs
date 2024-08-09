// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/apache/flink-kubernetes-operator/flink.apache.org/v1beta1/flinksessionjobs.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "flink.apache.org", version = "v1beta1", kind = "FlinkSessionJob", plural = "flinksessionjobs")]
#[kube(namespaced)]
#[kube(status = "FlinkSessionJobStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct FlinkSessionJobSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deploymentName")]
    pub deployment_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flinkConfiguration")]
    pub flink_configuration: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<FlinkSessionJobJob>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restartNonce")]
    pub restart_nonce: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlinkSessionJobJob {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowNonRestoredState")]
    pub allow_non_restored_state: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "checkpointTriggerNonce")]
    pub checkpoint_trigger_nonce: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "entryClass")]
    pub entry_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flinkStateSnapshotReference")]
    pub flink_state_snapshot_reference: Option<FlinkSessionJobJobFlinkStateSnapshotReference>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initialSavepointPath")]
    pub initial_savepoint_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jarURI")]
    pub jar_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "savepointRedeployNonce")]
    pub savepoint_redeploy_nonce: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "savepointTriggerNonce")]
    pub savepoint_trigger_nonce: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<FlinkSessionJobJobState>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upgradeMode")]
    pub upgrade_mode: Option<FlinkSessionJobJobUpgradeMode>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlinkSessionJobJobFlinkStateSnapshotReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlinkSessionJobJobState {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "suspended")]
    Suspended,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlinkSessionJobJobUpgradeMode {
    #[serde(rename = "last-state")]
    LastState,
    #[serde(rename = "savepoint")]
    Savepoint,
    #[serde(rename = "stateless")]
    Stateless,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlinkSessionJobStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobStatus")]
    pub job_status: Option<FlinkSessionJobStatusJobStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleState")]
    pub lifecycle_state: Option<FlinkSessionJobStatusLifecycleState>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconciliationStatus")]
    pub reconciliation_status: Option<FlinkSessionJobStatusReconciliationStatus>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlinkSessionJobStatusJobStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "checkpointInfo")]
    pub checkpoint_info: Option<FlinkSessionJobStatusJobStatusCheckpointInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobId")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobName")]
    pub job_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "savepointInfo")]
    pub savepoint_info: Option<FlinkSessionJobStatusJobStatusSavepointInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateTime")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upgradeSnapshotReference")]
    pub upgrade_snapshot_reference: Option<FlinkSessionJobStatusJobStatusUpgradeSnapshotReference>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlinkSessionJobStatusJobStatusCheckpointInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "formatType")]
    pub format_type: Option<FlinkSessionJobStatusJobStatusCheckpointInfoFormatType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastCheckpoint")]
    pub last_checkpoint: Option<FlinkSessionJobStatusJobStatusCheckpointInfoLastCheckpoint>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastPeriodicCheckpointTimestamp")]
    pub last_periodic_checkpoint_timestamp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerId")]
    pub trigger_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerTimestamp")]
    pub trigger_timestamp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerType")]
    pub trigger_type: Option<FlinkSessionJobStatusJobStatusCheckpointInfoTriggerType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlinkSessionJobStatusJobStatusCheckpointInfoFormatType {
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "INCREMENTAL")]
    Incremental,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlinkSessionJobStatusJobStatusCheckpointInfoLastCheckpoint {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "formatType")]
    pub format_type: Option<FlinkSessionJobStatusJobStatusCheckpointInfoLastCheckpointFormatType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeStamp")]
    pub time_stamp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerNonce")]
    pub trigger_nonce: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerType")]
    pub trigger_type: Option<FlinkSessionJobStatusJobStatusCheckpointInfoLastCheckpointTriggerType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlinkSessionJobStatusJobStatusCheckpointInfoLastCheckpointFormatType {
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "INCREMENTAL")]
    Incremental,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlinkSessionJobStatusJobStatusCheckpointInfoLastCheckpointTriggerType {
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "PERIODIC")]
    Periodic,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "UPGRADE")]
    Upgrade,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlinkSessionJobStatusJobStatusCheckpointInfoTriggerType {
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "PERIODIC")]
    Periodic,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "UPGRADE")]
    Upgrade,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlinkSessionJobStatusJobStatusSavepointInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "formatType")]
    pub format_type: Option<FlinkSessionJobStatusJobStatusSavepointInfoFormatType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastPeriodicSavepointTimestamp")]
    pub last_periodic_savepoint_timestamp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSavepoint")]
    pub last_savepoint: Option<FlinkSessionJobStatusJobStatusSavepointInfoLastSavepoint>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "savepointHistory")]
    pub savepoint_history: Option<Vec<FlinkSessionJobStatusJobStatusSavepointInfoSavepointHistory>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerId")]
    pub trigger_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerTimestamp")]
    pub trigger_timestamp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerType")]
    pub trigger_type: Option<FlinkSessionJobStatusJobStatusSavepointInfoTriggerType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlinkSessionJobStatusJobStatusSavepointInfoFormatType {
    #[serde(rename = "CANONICAL")]
    Canonical,
    #[serde(rename = "NATIVE")]
    Native,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlinkSessionJobStatusJobStatusSavepointInfoLastSavepoint {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "formatType")]
    pub format_type: Option<FlinkSessionJobStatusJobStatusSavepointInfoLastSavepointFormatType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeStamp")]
    pub time_stamp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerNonce")]
    pub trigger_nonce: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerType")]
    pub trigger_type: Option<FlinkSessionJobStatusJobStatusSavepointInfoLastSavepointTriggerType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlinkSessionJobStatusJobStatusSavepointInfoLastSavepointFormatType {
    #[serde(rename = "CANONICAL")]
    Canonical,
    #[serde(rename = "NATIVE")]
    Native,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlinkSessionJobStatusJobStatusSavepointInfoLastSavepointTriggerType {
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "PERIODIC")]
    Periodic,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "UPGRADE")]
    Upgrade,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlinkSessionJobStatusJobStatusSavepointInfoSavepointHistory {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "formatType")]
    pub format_type: Option<FlinkSessionJobStatusJobStatusSavepointInfoSavepointHistoryFormatType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeStamp")]
    pub time_stamp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerNonce")]
    pub trigger_nonce: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerType")]
    pub trigger_type: Option<FlinkSessionJobStatusJobStatusSavepointInfoSavepointHistoryTriggerType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlinkSessionJobStatusJobStatusSavepointInfoSavepointHistoryFormatType {
    #[serde(rename = "CANONICAL")]
    Canonical,
    #[serde(rename = "NATIVE")]
    Native,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlinkSessionJobStatusJobStatusSavepointInfoSavepointHistoryTriggerType {
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "PERIODIC")]
    Periodic,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "UPGRADE")]
    Upgrade,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlinkSessionJobStatusJobStatusSavepointInfoTriggerType {
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "PERIODIC")]
    Periodic,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "UPGRADE")]
    Upgrade,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlinkSessionJobStatusJobStatusUpgradeSnapshotReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlinkSessionJobStatusLifecycleState {
    #[serde(rename = "CREATED")]
    Created,
    #[serde(rename = "DEPLOYED")]
    Deployed,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "ROLLED_BACK")]
    RolledBack,
    #[serde(rename = "ROLLING_BACK")]
    RollingBack,
    #[serde(rename = "STABLE")]
    Stable,
    #[serde(rename = "SUSPENDED")]
    Suspended,
    #[serde(rename = "UPGRADING")]
    Upgrading,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlinkSessionJobStatusReconciliationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastReconciledSpec")]
    pub last_reconciled_spec: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastStableSpec")]
    pub last_stable_spec: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconciliationTimestamp")]
    pub reconciliation_timestamp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<FlinkSessionJobStatusReconciliationStatusState>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FlinkSessionJobStatusReconciliationStatusState {
    #[serde(rename = "DEPLOYED")]
    Deployed,
    #[serde(rename = "ROLLED_BACK")]
    RolledBack,
    #[serde(rename = "ROLLING_BACK")]
    RollingBack,
    #[serde(rename = "UPGRADING")]
    Upgrading,
}

