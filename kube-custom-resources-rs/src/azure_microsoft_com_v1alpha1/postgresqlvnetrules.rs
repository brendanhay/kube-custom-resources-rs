// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/Azure/azure-service-operator/azure.microsoft.com/v1alpha1/postgresqlvnetrules.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// PotgreSQLVNetRuleSpec defines the desired state of PostgreSQLVNetRule
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "azure.microsoft.com", version = "v1alpha1", kind = "PostgreSQLVNetRule", plural = "postgresqlvnetrules")]
#[kube(namespaced)]
#[kube(status = "PostgreSQLVNetRuleStatus")]
#[kube(schema = "disabled")]
pub struct PostgreSQLVNetRuleSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ignoreMissingServiceEndpoint")]
    pub ignore_missing_service_endpoint: Option<bool>,
    /// INSERT ADDITIONAL SPEC FIELDS - desired state of cluster Important: Run "make" to regenerate code after modifying this file
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    pub server: String,
    #[serde(rename = "subnetName")]
    pub subnet_name: String,
    #[serde(rename = "vNetName")]
    pub v_net_name: String,
    #[serde(rename = "vNetResourceGroup")]
    pub v_net_resource_group: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vNetSubscriptionID")]
    pub v_net_subscription_id: Option<String>,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostgreSQLVNetRuleStatus {
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
    pub polling_url_kind: Option<PostgreSQLVNetRuleStatusPollingUrlKind>,
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
pub enum PostgreSQLVNetRuleStatusPollingUrlKind {
    CreateOrUpdate,
    Delete,
}

