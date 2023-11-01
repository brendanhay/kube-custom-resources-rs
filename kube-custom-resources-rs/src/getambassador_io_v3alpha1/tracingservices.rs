// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/emissary-ingress/emissary/getambassador.io/v3alpha1/tracingservices.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// TracingServiceSpec defines the desired state of TracingService
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "getambassador.io", version = "v3alpha1", kind = "TracingService", plural = "tracingservices")]
#[kube(namespaced)]
pub struct TracingServiceSpec {
    /// AmbassadorID declares which Ambassador instances should pay attention to this resource. If no value is provided, the default is: 
    ///  ambassador_id: - "default"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ambassador_id: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<TracingServiceConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_tags: Option<Vec<TracingServiceCustomTags>>,
    pub driver: TracingServiceDriver,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sampling: Option<TracingServiceSampling>,
    pub service: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats_name: Option<String>,
    /// Deprecated: tag_headers is deprecated. Use custom_tags instead. `tag_headers: ["header"]` can be defined as `custom_tags: [{"request_header": {"name": "header"}}]`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_headers: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TracingServiceConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_token_file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collector_cluster: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collector_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collector_endpoint_version: Option<TracingServiceConfigCollectorEndpointVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collector_hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub propagation_modes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared_span_context: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id_128bit: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum TracingServiceConfigCollectorEndpointVersion {
    #[serde(rename = "HTTP_JSON_V1")]
    HttpJsonV1,
    #[serde(rename = "HTTP_JSON")]
    HttpJson,
    #[serde(rename = "HTTP_PROTO")]
    HttpProto,
}

/// TracingCustomTag provides a data structure for capturing envoy's `type.tracing.v3.CustomTag`
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TracingServiceCustomTags {
    /// Environment explicitly specifies the protocol stack to set up. Exactly one of Literal, Environment or Header must be supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<TracingServiceCustomTagsEnvironment>,
    /// Literal explicitly specifies the protocol stack to set up. Exactly one of Literal, Environment or Header must be supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub literal: Option<TracingServiceCustomTagsLiteral>,
    /// Header explicitly specifies the protocol stack to set up. Exactly one of Literal, Environment or Header must be supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_header: Option<TracingServiceCustomTagsRequestHeader>,
    pub tag: String,
}

/// Environment explicitly specifies the protocol stack to set up. Exactly one of Literal, Environment or Header must be supplied.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TracingServiceCustomTagsEnvironment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    pub name: String,
}

/// Literal explicitly specifies the protocol stack to set up. Exactly one of Literal, Environment or Header must be supplied.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TracingServiceCustomTagsLiteral {
    pub value: String,
}

/// Header explicitly specifies the protocol stack to set up. Exactly one of Literal, Environment or Header must be supplied.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TracingServiceCustomTagsRequestHeader {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    pub name: String,
}

/// TracingServiceSpec defines the desired state of TracingService
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum TracingServiceDriver {
    #[serde(rename = "lightstep")]
    Lightstep,
    #[serde(rename = "zipkin")]
    Zipkin,
    #[serde(rename = "datadog")]
    Datadog,
    #[serde(rename = "opentelemetry")]
    Opentelemetry,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TracingServiceSampling {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overall: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub random: Option<i64>,
}

