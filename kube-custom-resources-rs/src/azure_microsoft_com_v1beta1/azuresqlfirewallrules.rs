// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/Azure/azure-service-operator/azure.microsoft.com/v1beta1/azuresqlfirewallrules.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// AzureSqlFirewallRuleSpec defines the desired state of AzureSqlFirewallRule
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "azure.microsoft.com", version = "v1beta1", kind = "AzureSqlFirewallRule", plural = "azuresqlfirewallrules")]
#[kube(namespaced)]
#[kube(status = "AzureSqlFirewallRuleStatus")]
#[kube(schema = "disabled")]
pub struct AzureSqlFirewallRuleSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endIpAddress")]
    pub end_ip_address: Option<String>,
    /// INSERT ADDITIONAL SPEC FIELDS - desired state of cluster Important: Run "make" to regenerate code after modifying this file
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    pub server: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startIpAddress")]
    pub start_ip_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subscriptionID")]
    pub subscription_id: Option<String>,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AzureSqlFirewallRuleStatus {
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
    pub polling_url_kind: Option<AzureSqlFirewallRuleStatusPollingUrlKind>,
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
pub enum AzureSqlFirewallRuleStatusPollingUrlKind {
    CreateOrUpdate,
    Delete,
}

