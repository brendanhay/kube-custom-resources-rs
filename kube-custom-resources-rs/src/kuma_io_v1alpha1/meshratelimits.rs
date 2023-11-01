// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/kumahq/kuma/kuma.io/v1alpha1/meshratelimits.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Spec is the specification of the Kuma MeshRateLimit resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "kuma.io", version = "v1alpha1", kind = "MeshRateLimit", plural = "meshratelimits")]
#[kube(namespaced)]
pub struct MeshRateLimitSpec {
    /// From list makes a match between clients and corresponding configurations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<MeshRateLimitFrom>>,
    /// TargetRef is a reference to the resource the policy takes an effect on. The resource could be either a real store object or virtual resource defined inplace.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshRateLimitTargetRef,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MeshRateLimitFrom {
    /// Default is a configuration specific to the group of clients referenced in 'targetRef'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<MeshRateLimitFromDefault>,
    /// TargetRef is a reference to the resource that represents a group of clients.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshRateLimitFromTargetRef,
}

/// Default is a configuration specific to the group of clients referenced in 'targetRef'
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MeshRateLimitFromDefault {
    /// LocalConf defines local http or/and tcp rate limit configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<MeshRateLimitFromDefaultLocal>,
}

/// LocalConf defines local http or/and tcp rate limit configuration
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MeshRateLimitFromDefaultLocal {
    /// LocalHTTP defines confguration of local HTTP rate limiting https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/local_rate_limit_filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<MeshRateLimitFromDefaultLocalHttp>,
    /// LocalTCP defines confguration of local TCP rate limiting https://www.envoyproxy.io/docs/envoy/latest/configuration/listeners/network_filters/local_rate_limit_filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<MeshRateLimitFromDefaultLocalTcp>,
}

/// LocalHTTP defines confguration of local HTTP rate limiting https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/local_rate_limit_filter
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MeshRateLimitFromDefaultLocalHttp {
    /// Define if rate limiting should be disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Describes the actions to take on a rate limit event
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onRateLimit")]
    pub on_rate_limit: Option<MeshRateLimitFromDefaultLocalHttpOnRateLimit>,
    /// Defines how many requests are allowed per interval.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestRate")]
    pub request_rate: Option<MeshRateLimitFromDefaultLocalHttpRequestRate>,
}

/// Describes the actions to take on a rate limit event
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MeshRateLimitFromDefaultLocalHttpOnRateLimit {
    /// The Headers to be added to the HTTP response on a rate limit event
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<MeshRateLimitFromDefaultLocalHttpOnRateLimitHeaders>,
    /// The HTTP status code to be set on a rate limit event
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// The Headers to be added to the HTTP response on a rate limit event
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MeshRateLimitFromDefaultLocalHttpOnRateLimitHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<MeshRateLimitFromDefaultLocalHttpOnRateLimitHeadersAdd>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<MeshRateLimitFromDefaultLocalHttpOnRateLimitHeadersSet>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MeshRateLimitFromDefaultLocalHttpOnRateLimitHeadersAdd {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MeshRateLimitFromDefaultLocalHttpOnRateLimitHeadersSet {
    pub name: String,
    pub value: String,
}

/// Defines how many requests are allowed per interval.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MeshRateLimitFromDefaultLocalHttpRequestRate {
    /// The interval the number of units is accounted for.
    pub interval: String,
    /// Number of units per interval (depending on usage it can be a number of requests, or a number of connections).
    pub num: i32,
}

/// LocalTCP defines confguration of local TCP rate limiting https://www.envoyproxy.io/docs/envoy/latest/configuration/listeners/network_filters/local_rate_limit_filter
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MeshRateLimitFromDefaultLocalTcp {
    /// Defines how many connections are allowed per interval.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectionRate")]
    pub connection_rate: Option<MeshRateLimitFromDefaultLocalTcpConnectionRate>,
    /// Define if rate limiting should be disabled. Default: false
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

/// Defines how many connections are allowed per interval.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MeshRateLimitFromDefaultLocalTcpConnectionRate {
    /// The interval the number of units is accounted for.
    pub interval: String,
    /// Number of units per interval (depending on usage it can be a number of requests, or a number of connections).
    pub num: i32,
}

/// TargetRef is a reference to the resource that represents a group of clients.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MeshRateLimitFromTargetRef {
    /// Kind of the referenced resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MeshRateLimitFromTargetRefKind>,
    /// Mesh is reserved for future use to identify cross mesh resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<String>,
    /// Name of the referenced resource. Can only be used with kinds: `MeshService`, `MeshServiceSubset` and `MeshGatewayRoute`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tags used to select a subset of proxies by tags. Can only be used with kinds `MeshSubset` and `MeshServiceSubset`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// TargetRef is a reference to the resource that represents a group of clients.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum MeshRateLimitFromTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
}

/// TargetRef is a reference to the resource the policy takes an effect on. The resource could be either a real store object or virtual resource defined inplace.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MeshRateLimitTargetRef {
    /// Kind of the referenced resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MeshRateLimitTargetRefKind>,
    /// Mesh is reserved for future use to identify cross mesh resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<String>,
    /// Name of the referenced resource. Can only be used with kinds: `MeshService`, `MeshServiceSubset` and `MeshGatewayRoute`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tags used to select a subset of proxies by tags. Can only be used with kinds `MeshSubset` and `MeshServiceSubset`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// TargetRef is a reference to the resource the policy takes an effect on. The resource could be either a real store object or virtual resource defined inplace.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum MeshRateLimitTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
}

