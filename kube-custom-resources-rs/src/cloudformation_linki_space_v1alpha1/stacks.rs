// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/linki/cloudformation-operator/cloudformation.linki.space/v1alpha1/stacks.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Defines the desired state of Stack
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "cloudformation.linki.space", version = "v1alpha1", kind = "Stack", plural = "stacks")]
#[kube(namespaced)]
#[kube(status = "StackStatus")]
pub struct StackSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
    pub template: String,
}

/// Defines the observed state of Stack
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct StackStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createdTime")]
    pub created_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<StackStatusResources>>,
    #[serde(rename = "stackID")]
    pub stack_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stackStatus")]
    pub stack_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatedTime")]
    pub updated_time: Option<String>,
}

/// Defines a resource provided/managed by a Stack and its current state
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct StackStatusResources {
    #[serde(rename = "logicalID")]
    pub logical_id: String,
    #[serde(rename = "physicalID")]
    pub physical_id: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusReason")]
    pub status_reason: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

