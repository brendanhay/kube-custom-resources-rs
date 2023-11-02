// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/metacontroller/metacontroller/metacontroller.k8s.io/v1alpha1/compositecontrollers.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "metacontroller.k8s.io", version = "v1alpha1", kind = "CompositeController", plural = "compositecontrollers")]
#[kube(status = "CompositeControllerStatus")]
#[kube(schema = "disabled")]
pub struct CompositeControllerSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "childResources")]
    pub child_resources: Option<Vec<CompositeControllerChildResources>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generateSelector")]
    pub generate_selector: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hooks: Option<CompositeControllerHooks>,
    #[serde(rename = "parentResource")]
    pub parent_resource: CompositeControllerParentResource,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resyncPeriodSeconds")]
    pub resync_period_seconds: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerChildResources {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub resource: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateStrategy")]
    pub update_strategy: Option<CompositeControllerChildResourcesUpdateStrategy>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerChildResourcesUpdateStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<CompositeControllerChildResourcesUpdateStrategyMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusChecks")]
    pub status_checks: Option<CompositeControllerChildResourcesUpdateStrategyStatusChecks>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CompositeControllerChildResourcesUpdateStrategyMethod {
    OnDelete,
    Recreate,
    InPlace,
    RollingRecreate,
    RollingInPlace,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerChildResourcesUpdateStrategyStatusChecks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<CompositeControllerChildResourcesUpdateStrategyStatusChecksConditions>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerChildResourcesUpdateStrategyStatusChecksConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customize: Option<CompositeControllerHooksCustomize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finalize: Option<CompositeControllerHooksFinalize>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "postUpdateChild")]
    pub post_update_child: Option<CompositeControllerHooksPostUpdateChild>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preUpdateChild")]
    pub pre_update_child: Option<CompositeControllerHooksPreUpdateChild>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<CompositeControllerHooksSync>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksCustomize {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<CompositeControllerHooksCustomizeVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<CompositeControllerHooksCustomizeWebhook>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CompositeControllerHooksCustomizeVersion {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksCustomizeWebhook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<CompositeControllerHooksCustomizeWebhookEtag>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Sets the json unmarshall mode. One of the 'loose' or 'strict'. In 'strict' mode additional checks are performed to detect unknown and duplicated fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseUnMarshallMode")]
    pub response_un_marshall_mode: Option<CompositeControllerHooksCustomizeWebhookResponseUnMarshallMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<CompositeControllerHooksCustomizeWebhookService>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksCustomizeWebhookEtag {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheCleanupSeconds")]
    pub cache_cleanup_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheTimeoutSeconds")]
    pub cache_timeout_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CompositeControllerHooksCustomizeWebhookResponseUnMarshallMode {
    #[serde(rename = "loose")]
    Loose,
    #[serde(rename = "strict")]
    Strict,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksCustomizeWebhookService {
    pub name: String,
    pub namespace: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksFinalize {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<CompositeControllerHooksFinalizeVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<CompositeControllerHooksFinalizeWebhook>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CompositeControllerHooksFinalizeVersion {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksFinalizeWebhook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<CompositeControllerHooksFinalizeWebhookEtag>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Sets the json unmarshall mode. One of the 'loose' or 'strict'. In 'strict' mode additional checks are performed to detect unknown and duplicated fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseUnMarshallMode")]
    pub response_un_marshall_mode: Option<CompositeControllerHooksFinalizeWebhookResponseUnMarshallMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<CompositeControllerHooksFinalizeWebhookService>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksFinalizeWebhookEtag {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheCleanupSeconds")]
    pub cache_cleanup_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheTimeoutSeconds")]
    pub cache_timeout_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CompositeControllerHooksFinalizeWebhookResponseUnMarshallMode {
    #[serde(rename = "loose")]
    Loose,
    #[serde(rename = "strict")]
    Strict,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksFinalizeWebhookService {
    pub name: String,
    pub namespace: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksPostUpdateChild {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<CompositeControllerHooksPostUpdateChildVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<CompositeControllerHooksPostUpdateChildWebhook>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CompositeControllerHooksPostUpdateChildVersion {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksPostUpdateChildWebhook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<CompositeControllerHooksPostUpdateChildWebhookEtag>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Sets the json unmarshall mode. One of the 'loose' or 'strict'. In 'strict' mode additional checks are performed to detect unknown and duplicated fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseUnMarshallMode")]
    pub response_un_marshall_mode: Option<CompositeControllerHooksPostUpdateChildWebhookResponseUnMarshallMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<CompositeControllerHooksPostUpdateChildWebhookService>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksPostUpdateChildWebhookEtag {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheCleanupSeconds")]
    pub cache_cleanup_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheTimeoutSeconds")]
    pub cache_timeout_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CompositeControllerHooksPostUpdateChildWebhookResponseUnMarshallMode {
    #[serde(rename = "loose")]
    Loose,
    #[serde(rename = "strict")]
    Strict,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksPostUpdateChildWebhookService {
    pub name: String,
    pub namespace: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksPreUpdateChild {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<CompositeControllerHooksPreUpdateChildVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<CompositeControllerHooksPreUpdateChildWebhook>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CompositeControllerHooksPreUpdateChildVersion {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksPreUpdateChildWebhook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<CompositeControllerHooksPreUpdateChildWebhookEtag>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Sets the json unmarshall mode. One of the 'loose' or 'strict'. In 'strict' mode additional checks are performed to detect unknown and duplicated fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseUnMarshallMode")]
    pub response_un_marshall_mode: Option<CompositeControllerHooksPreUpdateChildWebhookResponseUnMarshallMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<CompositeControllerHooksPreUpdateChildWebhookService>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksPreUpdateChildWebhookEtag {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheCleanupSeconds")]
    pub cache_cleanup_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheTimeoutSeconds")]
    pub cache_timeout_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CompositeControllerHooksPreUpdateChildWebhookResponseUnMarshallMode {
    #[serde(rename = "loose")]
    Loose,
    #[serde(rename = "strict")]
    Strict,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksPreUpdateChildWebhookService {
    pub name: String,
    pub namespace: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksSync {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<CompositeControllerHooksSyncVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<CompositeControllerHooksSyncWebhook>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CompositeControllerHooksSyncVersion {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksSyncWebhook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<CompositeControllerHooksSyncWebhookEtag>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Sets the json unmarshall mode. One of the 'loose' or 'strict'. In 'strict' mode additional checks are performed to detect unknown and duplicated fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseUnMarshallMode")]
    pub response_un_marshall_mode: Option<CompositeControllerHooksSyncWebhookResponseUnMarshallMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<CompositeControllerHooksSyncWebhookService>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksSyncWebhookEtag {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheCleanupSeconds")]
    pub cache_cleanup_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheTimeoutSeconds")]
    pub cache_timeout_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CompositeControllerHooksSyncWebhookResponseUnMarshallMode {
    #[serde(rename = "loose")]
    Loose,
    #[serde(rename = "strict")]
    Strict,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerHooksSyncWebhookService {
    pub name: String,
    pub namespace: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerParentResource {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<CompositeControllerParentResourceLabelSelector>,
    pub resource: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revisionHistory")]
    pub revision_history: Option<CompositeControllerParentResourceRevisionHistory>,
}

/// A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerParentResourceLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CompositeControllerParentResourceLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerParentResourceLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerParentResourceRevisionHistory {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPaths")]
    pub field_paths: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeControllerStatus {
}

