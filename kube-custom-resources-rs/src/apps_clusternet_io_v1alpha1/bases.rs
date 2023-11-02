// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/clusternet/clusternet/apps.clusternet.io/v1alpha1/bases.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// BaseSpec defines the desired state of Base
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "apps.clusternet.io", version = "v1alpha1", kind = "Base", plural = "bases")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct BaseSpec {
    /// Feeds
    pub feeds: Vec<BaseFeeds>,
}

/// Feed defines the resource to be selected.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BaseFeeds {
    /// APIVersion defines the versioned schema of this representation of an object.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind is a string value representing the REST resource this object represents. In CamelCase.
    pub kind: String,
    /// Name of the target resource.
    pub name: String,
    /// Namespace of the target resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

