// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/Azure/azure-service-operator/azure.microsoft.com/v1alpha1/appinsightsapikeys.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// AppInsightsApiKeySpec defines the desired state of AppInsightsApiKey
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "azure.microsoft.com", version = "v1alpha1", kind = "AppInsightsApiKey", plural = "appinsightsapikeys")]
#[kube(namespaced)]
#[kube(status = "AppInsightsApiKeyStatus")]
#[kube(schema = "disabled")]
pub struct AppInsightsApiKeySpec {
    #[serde(rename = "appInsights")]
    pub app_insights: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authSDKControlChannel")]
    pub auth_sdk_control_channel: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readTelemetry")]
    pub read_telemetry: Option<bool>,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "writeAnnotations")]
    pub write_annotations: Option<bool>,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppInsightsApiKeyStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containsUpdate")]
    pub contains_update: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failedProvisioning")]
    pub failed_provisioning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flattenedSecrets")]
    pub flattened_secrets: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pollingUrl")]
    pub polling_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pollingUrlKind")]
    pub polling_url_kind: Option<AppInsightsApiKeyStatusPollingUrlKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceId")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "specHash")]
    pub spec_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AppInsightsApiKeyStatusPollingUrlKind {
    CreateOrUpdate,
    Delete,
}

