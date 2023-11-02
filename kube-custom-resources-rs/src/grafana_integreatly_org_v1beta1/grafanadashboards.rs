// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/grafana-operator/grafana-operator/grafana.integreatly.org/v1beta1/grafanadashboards.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "grafana.integreatly.org", version = "v1beta1", kind = "GrafanaDashboard", plural = "grafanadashboards")]
#[kube(namespaced)]
#[kube(status = "GrafanaDashboardStatus")]
#[kube(schema = "disabled")]
pub struct GrafanaDashboardSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowCrossNamespaceImport")]
    pub allow_cross_namespace_import: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapRef")]
    pub config_map_ref: Option<GrafanaDashboardConfigMapRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contentCacheDuration")]
    pub content_cache_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datasources: Option<Vec<GrafanaDashboardDatasources>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "envFrom")]
    pub env_from: Option<Vec<GrafanaDashboardEnvFrom>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub envs: Option<Vec<GrafanaDashboardEnvs>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "grafanaCom")]
    pub grafana_com: Option<GrafanaDashboardGrafanaCom>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gzipJson")]
    pub gzip_json: Option<String>,
    #[serde(rename = "instanceSelector")]
    pub instance_selector: GrafanaDashboardInstanceSelector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jsonnet: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jsonnetLib")]
    pub jsonnet_lib: Option<GrafanaDashboardJsonnetLib>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<GrafanaDashboardPlugins>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resyncPeriod")]
    pub resync_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrafanaDashboardConfigMapRef {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrafanaDashboardDatasources {
    #[serde(rename = "datasourceName")]
    pub datasource_name: String,
    #[serde(rename = "inputName")]
    pub input_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrafanaDashboardEnvFrom {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<GrafanaDashboardEnvFromConfigMapKeyRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<GrafanaDashboardEnvFromSecretKeyRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrafanaDashboardEnvFromConfigMapKeyRef {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrafanaDashboardEnvFromSecretKeyRef {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrafanaDashboardEnvs {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<GrafanaDashboardEnvsValueFrom>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrafanaDashboardEnvsValueFrom {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<GrafanaDashboardEnvsValueFromConfigMapKeyRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<GrafanaDashboardEnvsValueFromSecretKeyRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrafanaDashboardEnvsValueFromConfigMapKeyRef {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrafanaDashboardEnvsValueFromSecretKeyRef {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrafanaDashboardGrafanaCom {
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrafanaDashboardInstanceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<GrafanaDashboardInstanceSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrafanaDashboardInstanceSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrafanaDashboardJsonnetLib {
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "gzipJsonnetProject")]
    pub gzip_jsonnet_project: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jPath")]
    pub j_path: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrafanaDashboardPlugins {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrafanaDashboardStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "NoMatchingInstances")]
    pub no_matching_instances: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contentCache")]
    pub content_cache: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contentTimestamp")]
    pub content_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contentUrl")]
    pub content_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastResync")]
    pub last_resync: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

