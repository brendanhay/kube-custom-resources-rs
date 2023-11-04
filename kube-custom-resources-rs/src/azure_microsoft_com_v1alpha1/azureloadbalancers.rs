// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/Azure/azure-service-operator/azure.microsoft.com/v1alpha1/azureloadbalancers.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// AzureLoadBalancerSpec defines the desired state of AzureLoadBalancer
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "azure.microsoft.com", version = "v1alpha1", kind = "AzureLoadBalancer", plural = "azureloadbalancers")]
#[kube(namespaced)]
#[kube(status = "AzureLoadBalancerStatus")]
#[kube(schema = "disabled")]
pub struct AzureLoadBalancerSpec {
    #[serde(rename = "backendAddressPoolName")]
    pub backend_address_pool_name: String,
    #[serde(rename = "backendPort")]
    pub backend_port: i64,
    #[serde(rename = "frontendPortRangeEnd")]
    pub frontend_port_range_end: i64,
    #[serde(rename = "frontendPortRangeStart")]
    pub frontend_port_range_start: i64,
    #[serde(rename = "inboundNatPoolName")]
    pub inbound_nat_pool_name: String,
    /// INSERT ADDITIONAL SPEC FIELDS - desired state of cluster Important: Run "make" to regenerate code after modifying this file
    pub location: String,
    #[serde(rename = "publicIPAddressName")]
    pub public_ip_address_name: String,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AzureLoadBalancerStatus {
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
    pub polling_url_kind: Option<AzureLoadBalancerStatusPollingUrlKind>,
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
pub enum AzureLoadBalancerStatusPollingUrlKind {
    CreateOrUpdate,
    Delete,
}
