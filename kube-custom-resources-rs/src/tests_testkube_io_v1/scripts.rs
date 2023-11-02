// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/kubeshop/testkube-operator/tests.testkube.io/v1/scripts.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// ScriptSpec defines the desired state of Script
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "tests.testkube.io", version = "v1", kind = "Script", plural = "scripts")]
#[kube(namespaced)]
#[kube(status = "ScriptStatus")]
#[kube(schema = "disabled")]
pub struct ScriptSpec {
    /// script content as string (content depends from executor)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// script content type can be:  - direct content - created from file, - git repo directory checkout in case when test is some kind of project or have more than one file,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "input-type")]
    pub input_type: Option<String>,
    /// script execution custom name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// execution params passed to executor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<BTreeMap<String, String>>,
    /// repository details if exists
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<ScriptRepository>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// script type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// repository details if exists
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScriptRepository {
    /// branch/tag name for checkout
    pub branch: String,
    /// if needed we can checkout particular path (dir or file) in case of BIG/mono repositories
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// git auth token for private repositories
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Type_ repository type
    #[serde(rename = "type")]
    pub r#type: String,
    /// Uri of content file or git directory
    pub uri: String,
    /// git auth username for private repositories
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// ScriptStatus defines the observed state of Script
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScriptStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executions_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_execution: Option<String>,
}

