// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/grafana-operator/grafana-operator/grafana.integreatly.org/v1beta1/grafanadatasources.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "grafana.integreatly.org", version = "v1beta1", kind = "GrafanaDatasource", plural = "grafanadatasources")]
#[kube(namespaced)]
#[kube(status = "GrafanaDatasourceStatus")]
#[kube(schema = "disabled")]
pub struct GrafanaDatasourceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowCrossNamespaceImport")]
    pub allow_cross_namespace_import: Option<bool>,
    pub datasource: GrafanaDatasourceDatasource,
    #[serde(rename = "instanceSelector")]
    pub instance_selector: GrafanaDatasourceInstanceSelector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<GrafanaDatasourcePlugins>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resyncPeriod")]
    pub resync_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valuesFrom")]
    pub values_from: Option<Vec<GrafanaDatasourceValuesFrom>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaDatasourceDatasource {
    pub access: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "basicAuth")]
    pub basic_auth: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "basicAuthUser")]
    pub basic_auth_user: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isDefault")]
    pub is_default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jsonData")]
    pub json_data: Option<BTreeMap<String, serde_json::Value>>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "orgId")]
    pub org_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secureJsonData")]
    pub secure_json_data: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaDatasourceInstanceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<GrafanaDatasourceInstanceSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaDatasourceInstanceSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaDatasourcePlugins {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaDatasourceValuesFrom {
    #[serde(rename = "targetPath")]
    pub target_path: String,
    #[serde(rename = "valueFrom")]
    pub value_from: GrafanaDatasourceValuesFromValueFrom,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaDatasourceValuesFromValueFrom {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<GrafanaDatasourceValuesFromValueFromConfigMapKeyRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<GrafanaDatasourceValuesFromValueFromSecretKeyRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaDatasourceValuesFromValueFromConfigMapKeyRef {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaDatasourceValuesFromValueFromSecretKeyRef {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaDatasourceStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "NoMatchingInstances")]
    pub no_matching_instances: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastMessage")]
    pub last_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastResync")]
    pub last_resync: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

