// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws/aws-application-networking-k8s/application-networking.k8s.aws/v1alpha1/serviceexports.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// status describes the current state of an exported service. Service configuration comes from the Service that had the same name and namespace as this ServiceExport. Populated by the multi-cluster service implementation's controller.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ServiceExportStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ServiceExportStatusConditions>>,
}

/// ServiceExportCondition contains details for the current condition of this service export. 
///  Once [KEP-1623](https://github.com/kubernetes/enhancements/tree/master/keps/sig-api-machinery/1623-standardize-conditions) is implemented, this will be replaced by metav1.Condition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ServiceExportStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status is one of {"True", "False", "Unknown"}
    pub status: ServiceExportStatusConditionsStatus,
    /// ServiceExportConditionType identifies a specific condition.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// ServiceExportCondition contains details for the current condition of this service export. 
///  Once [KEP-1623](https://github.com/kubernetes/enhancements/tree/master/keps/sig-api-machinery/1623-standardize-conditions) is implemented, this will be replaced by metav1.Condition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ServiceExportStatusConditionsStatus {
    True,
    False,
    Unknown,
}

