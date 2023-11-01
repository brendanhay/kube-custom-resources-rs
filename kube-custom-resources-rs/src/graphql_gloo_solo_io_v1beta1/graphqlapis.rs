// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/solo-io/gloo/graphql.gloo.solo.io/v1beta1/graphqlapis.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "graphql.gloo.solo.io", version = "v1beta1", kind = "GraphQLApi", plural = "graphqlapis")]
#[kube(namespaced)]
#[kube(status = "GraphQLApiStatus")]
pub struct GraphQLApiSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedQueryHashes")]
    pub allowed_query_hashes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "executableSchema")]
    pub executable_schema: Option<GraphQLApiExecutableSchema>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespacedStatuses")]
    pub namespaced_statuses: Option<GraphQLApiNamespacedStatuses>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<GraphQLApiOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "persistedQueryCacheConfig")]
    pub persisted_query_cache_config: Option<GraphQLApiPersistedQueryCacheConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statPrefix")]
    pub stat_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stitchedSchema")]
    pub stitched_schema: Option<GraphQLApiStitchedSchema>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executor: Option<GraphQLApiExecutableSchemaExecutor>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "grpcDescriptorRegistry")]
    pub grpc_descriptor_registry: Option<GraphQLApiExecutableSchemaGrpcDescriptorRegistry>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "schemaDefinition")]
    pub schema_definition: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaExecutor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<GraphQLApiExecutableSchemaExecutorLocal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote: Option<GraphQLApiExecutableSchemaExecutorRemote>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaExecutorLocal {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableIntrospection")]
    pub enable_introspection: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<GraphQLApiExecutableSchemaExecutorLocalOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolutions: Option<BTreeMap<String, GraphQLApiExecutableSchemaExecutorLocalResolutions>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaExecutorLocalOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxDepth")]
    pub max_depth: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaExecutorLocalResolutions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "grpcResolver")]
    pub grpc_resolver: Option<GraphQLApiExecutableSchemaExecutorLocalResolutionsGrpcResolver>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mockResolver")]
    pub mock_resolver: Option<GraphQLApiExecutableSchemaExecutorLocalResolutionsMockResolver>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restResolver")]
    pub rest_resolver: Option<GraphQLApiExecutableSchemaExecutorLocalResolutionsRestResolver>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statPrefix")]
    pub stat_prefix: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaExecutorLocalResolutionsGrpcResolver {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestTransform")]
    pub request_transform: Option<GraphQLApiExecutableSchemaExecutorLocalResolutionsGrpcResolverRequestTransform>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "spanName")]
    pub span_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upstreamRef")]
    pub upstream_ref: Option<GraphQLApiExecutableSchemaExecutorLocalResolutionsGrpcResolverUpstreamRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaExecutorLocalResolutionsGrpcResolverRequestTransform {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "methodName")]
    pub method_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outgoingMessageJson")]
    pub outgoing_message_json: Option<HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestMetadata")]
    pub request_metadata: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceName")]
    pub service_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaExecutorLocalResolutionsGrpcResolverUpstreamRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaExecutorLocalResolutionsMockResolver {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "asyncResponse")]
    pub async_response: Option<GraphQLApiExecutableSchemaExecutorLocalResolutionsMockResolverAsyncResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorResponse")]
    pub error_response: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncResponse")]
    pub sync_response: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaExecutorLocalResolutionsMockResolverAsyncResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaExecutorLocalResolutionsRestResolver {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<GraphQLApiExecutableSchemaExecutorLocalResolutionsRestResolverRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<GraphQLApiExecutableSchemaExecutorLocalResolutionsRestResolverResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "spanName")]
    pub span_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upstreamRef")]
    pub upstream_ref: Option<GraphQLApiExecutableSchemaExecutorLocalResolutionsRestResolverUpstreamRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaExecutorLocalResolutionsRestResolverRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryParams")]
    pub query_params: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaExecutorLocalResolutionsRestResolverResponse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resultRoot")]
    pub result_root: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub setters: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaExecutorLocalResolutionsRestResolverUpstreamRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaExecutorRemote {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryParams")]
    pub query_params: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "spanName")]
    pub span_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upstreamRef")]
    pub upstream_ref: Option<GraphQLApiExecutableSchemaExecutorRemoteUpstreamRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaExecutorRemoteUpstreamRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaGrpcDescriptorRegistry {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protoDescriptor")]
    pub proto_descriptor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protoDescriptorBin")]
    pub proto_descriptor_bin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protoRefsList")]
    pub proto_refs_list: Option<GraphQLApiExecutableSchemaGrpcDescriptorRegistryProtoRefsList>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaGrpcDescriptorRegistryProtoRefsList {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapRefs")]
    pub config_map_refs: Option<Vec<GraphQLApiExecutableSchemaGrpcDescriptorRegistryProtoRefsListConfigMapRefs>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiExecutableSchemaGrpcDescriptorRegistryProtoRefsListConfigMapRefs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiNamespacedStatuses {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<BTreeMap<String, GraphQLApiNamespacedStatusesStatuses>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logSensitiveInfo")]
    pub log_sensitive_info: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiPersistedQueryCacheConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheSize")]
    pub cache_size: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiStitchedSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subschemas: Option<Vec<GraphQLApiStitchedSchemaSubschemas>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiStitchedSchemaSubschemas {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "typeMerge")]
    pub type_merge: Option<BTreeMap<String, GraphQLApiStitchedSchemaSubschemasTypeMerge>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiStitchedSchemaSubschemasTypeMerge {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryName")]
    pub query_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "selectionSet")]
    pub selection_set: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GraphQLApiStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<BTreeMap<String, serde_json::Value>>,
}

