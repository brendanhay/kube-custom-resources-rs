// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/Azure/azure-service-operator/azure.microsoft.com/v1alpha1/azurepublicipaddresses.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// AzurePublicIPAddressSpec defines the desired state of AzurePublicIPAddress
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "azure.microsoft.com", version = "v1alpha1", kind = "AzurePublicIPAddress", plural = "azurepublicipaddresses")]
#[kube(namespaced)]
#[kube(status = "AzurePublicIPAddressStatus")]
#[kube(schema = "disabled")]
pub struct AzurePublicIPAddressSpec {
    #[serde(rename = "idleTimeoutInMinutes")]
    pub idle_timeout_in_minutes: i64,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipTags")]
    pub ip_tags: Option<BTreeMap<String, String>>,
    pub location: String,
    #[serde(rename = "publicIPAddressVersion")]
    pub public_ip_address_version: String,
    #[serde(rename = "publicIPAllocationMethod")]
    pub public_ip_allocation_method: String,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[serde(rename = "skuName")]
    pub sku_name: String,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AzurePublicIPAddressStatus {
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
    pub polling_url_kind: Option<AzurePublicIPAddressStatusPollingUrlKind>,
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
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AzurePublicIPAddressStatusPollingUrlKind {
    CreateOrUpdate,
    Delete,
}
