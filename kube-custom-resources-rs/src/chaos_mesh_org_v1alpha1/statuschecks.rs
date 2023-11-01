// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/chaos-mesh/chaos-mesh/chaos-mesh.org/v1alpha1/statuschecks.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Spec defines the behavior of a status check
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "chaos-mesh.org", version = "v1alpha1", kind = "StatusCheck", plural = "statuschecks")]
#[kube(namespaced)]
#[kube(status = "StatusCheckStatus")]
pub struct StatusCheckSpec {
    /// Duration defines the duration of the whole status check if the number of failed execution does not exceed the failure threshold. Duration is available to both `Synchronous` and `Continuous` mode. A duration string is a possibly signed sequence of decimal numbers, each with optional fraction and a unit suffix, such as "300ms", "-1.5h" or "2h45m". Valid time units are "ns", "us" (or "µs"), "ms", "s", "m", "h".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// FailureThreshold defines the minimum consecutive failure for the status check to be considered failed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureThreshold")]
    pub failure_threshold: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<StatusCheckHttp>,
    /// IntervalSeconds defines how often (in seconds) to perform an execution of status check.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "intervalSeconds")]
    pub interval_seconds: Option<i64>,
    /// Mode defines the execution mode of the status check. Support type: Synchronous / Continuous
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<StatusCheckMode>,
    /// RecordsHistoryLimit defines the number of record to retain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "recordsHistoryLimit")]
    pub records_history_limit: Option<i64>,
    /// SuccessThreshold defines the minimum consecutive successes for the status check to be considered successful. SuccessThreshold only works for `Synchronous` mode.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "successThreshold")]
    pub success_threshold: Option<i64>,
    /// TimeoutSeconds defines the number of seconds after which an execution of status check times out.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeoutSeconds")]
    pub timeout_seconds: Option<i64>,
    /// Type defines the specific status check type. Support type: HTTP
    #[serde(rename = "type")]
    pub r#type: StatusCheckType,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct StatusCheckHttp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Criteria defines how to determine the result of the status check.
    pub criteria: StatusCheckHttpCriteria,
    /// A Header represents the key-value pairs in an HTTP header. 
    ///  The keys should be in canonical form, as returned by CanonicalHeaderKey.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<StatusCheckHttpMethod>,
    pub url: String,
}

/// Criteria defines how to determine the result of the status check.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct StatusCheckHttpCriteria {
    /// StatusCode defines the expected http status code for the request. A statusCode string could be a single code (e.g. 200), or an inclusive range (e.g. 200-400, both `200` and `400` are included).
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum StatusCheckHttpMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
}

/// Spec defines the behavior of a status check
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum StatusCheckMode {
    Synchronous,
    Continuous,
}

/// Spec defines the behavior of a status check
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum StatusCheckType {
    #[serde(rename = "HTTP")]
    Http,
}

/// Most recently observed status of status check
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct StatusCheckStatus {
    /// CompletionTime represents time when the status check was completed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "completionTime")]
    pub completion_time: Option<String>,
    /// Conditions represents the latest available observations of a StatusCheck's current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<StatusCheckStatusConditions>>,
    /// Count represents the total number of the status check executed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Records contains the history of the execution of StatusCheck.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<StatusCheckStatusRecords>>,
    /// StartTime represents time when the status check started to execute.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct StatusCheckStatusConditions {
    #[serde(rename = "lastProbeTime")]
    pub last_probe_time: String,
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    pub reason: String,
    pub status: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct StatusCheckStatusRecords {
    pub outcome: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
}

