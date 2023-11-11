// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/apecloud/kubeblocks/dataprotection.kubeblocks.io/v1alpha1/backupschedules.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// BackupScheduleSpec defines the desired state of BackupSchedule.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "dataprotection.kubeblocks.io", version = "v1alpha1", kind = "BackupSchedule", plural = "backupschedules")]
#[kube(namespaced)]
#[kube(status = "BackupScheduleStatus")]
#[kube(schema = "disabled")]
pub struct BackupScheduleSpec {
    /// Which backupPolicy is applied to perform this backup.
    #[serde(rename = "backupPolicyName")]
    pub backup_policy_name: String,
    /// schedules defines the list of backup schedules.
    pub schedules: Vec<BackupScheduleSchedules>,
    /// startingDeadlineMinutes defines the deadline in minutes for starting the backup workload if it misses scheduled time for any reason.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startingDeadlineMinutes")]
    pub starting_deadline_minutes: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupScheduleSchedules {
    /// backupMethod specifies the backup method name that is defined in backupPolicy.
    #[serde(rename = "backupMethod")]
    pub backup_method: String,
    /// the cron expression for schedule, the timezone is in UTC. see https://en.wikipedia.org/wiki/Cron.
    #[serde(rename = "cronExpression")]
    pub cron_expression: String,
    /// enabled specifies whether the backup schedule is enabled or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// retentionPeriod determines a duration up to which the backup should be kept. controller will remove all backups that are older than the RetentionPeriod. For example, RetentionPeriod of `30d` will keep only the backups of last 30 days. Sample duration format: - years: 	2y - months: 	6mo - days: 		30d - hours: 	12h - minutes: 	30m You can also combine the above durations. For example: 30d12h30m
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retentionPeriod")]
    pub retention_period: Option<String>,
}

/// BackupScheduleStatus defines the observed state of BackupSchedule.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupScheduleStatus {
    /// failureReason is an error that caused the backup to fail.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// observedGeneration is the most recent generation observed for this BackupSchedule. It refers to the BackupSchedule's generation, which is updated on mutation by the API Server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// phase describes the phase of the BackupSchedule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// schedules describes the status of each schedule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedules: Option<BTreeMap<String, BackupScheduleStatusSchedules>>,
}

/// schedules describes the status of each schedule.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupScheduleStatusSchedules {
    /// failureReason is an error that caused the backup to fail.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// lastScheduleTime records the last time the backup was scheduled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastScheduleTime")]
    pub last_schedule_time: Option<String>,
    /// lastSuccessfulTime records the last time the backup was successfully completed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSuccessfulTime")]
    pub last_successful_time: Option<String>,
    /// phase describes the phase of the schedule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

