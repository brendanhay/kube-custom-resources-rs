// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/kumahq/kuma/kuma.io/v1alpha1/meshaccesslogs.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::collections::HashMap;

/// Spec is the specification of the Kuma MeshAccessLog resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "kuma.io", version = "v1alpha1", kind = "MeshAccessLog", plural = "meshaccesslogs")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct MeshAccessLogSpec {
    /// From list makes a match between clients and corresponding configurations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<MeshAccessLogFrom>>,
    /// TargetRef is a reference to the resource the policy takes an effect on. The resource could be either a real store object or virtual resource defined inplace.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshAccessLogTargetRef,
    /// To list makes a match between the consumed services and corresponding configurations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<MeshAccessLogTo>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogFrom {
    /// Default is a configuration specific to the group of clients referenced in 'targetRef'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<MeshAccessLogFromDefault>,
    /// TargetRef is a reference to the resource that represents a group of clients.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshAccessLogFromTargetRef,
}

/// Default is a configuration specific to the group of clients referenced in 'targetRef'
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogFromDefault {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backends: Option<Vec<MeshAccessLogFromDefaultBackends>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogFromDefaultBackends {
    /// FileBackend defines configuration for file based access logs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<MeshAccessLogFromDefaultBackendsFile>,
    /// Defines an OpenTelemetry logging backend.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openTelemetry")]
    pub open_telemetry: Option<MeshAccessLogFromDefaultBackendsOpenTelemetry>,
    /// TCPBackend defines a TCP logging backend.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<MeshAccessLogFromDefaultBackendsTcp>,
    #[serde(rename = "type")]
    pub r#type: MeshAccessLogFromDefaultBackendsType,
}

/// FileBackend defines configuration for file based access logs
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogFromDefaultBackendsFile {
    /// Format of access logs. Placeholders available on https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage#command-operators
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<MeshAccessLogFromDefaultBackendsFileFormat>,
    /// Path to a file that logs will be written to
    pub path: String,
}

/// Format of access logs. Placeholders available on https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage#command-operators
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogFromDefaultBackendsFileFormat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json: Option<Vec<MeshAccessLogFromDefaultBackendsFileFormatJson>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "omitEmptyValues")]
    pub omit_empty_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plain: Option<String>,
    #[serde(rename = "type")]
    pub r#type: MeshAccessLogFromDefaultBackendsFileFormatType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogFromDefaultBackendsFileFormatJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Format of access logs. Placeholders available on https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage#command-operators
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MeshAccessLogFromDefaultBackendsFileFormatType {
    Plain,
    Json,
}

/// Defines an OpenTelemetry logging backend.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogFromDefaultBackendsOpenTelemetry {
    /// Attributes can contain placeholders available on https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage#command-operators
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<MeshAccessLogFromDefaultBackendsOpenTelemetryAttributes>>,
    /// Body is a raw string or an OTLP any value as described at https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/logs/data-model.md#field-body It can contain placeholders available on https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage#command-operators
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<HashMap<String, serde_json::Value>>,
    /// Endpoint of OpenTelemetry collector. An empty port defaults to 4317.
    pub endpoint: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogFromDefaultBackendsOpenTelemetryAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// TCPBackend defines a TCP logging backend.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogFromDefaultBackendsTcp {
    /// Address of the TCP logging backend
    pub address: String,
    /// Format of access logs. Placeholders available on https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage#command-operators
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<MeshAccessLogFromDefaultBackendsTcpFormat>,
}

/// Format of access logs. Placeholders available on https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage#command-operators
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogFromDefaultBackendsTcpFormat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json: Option<Vec<MeshAccessLogFromDefaultBackendsTcpFormatJson>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "omitEmptyValues")]
    pub omit_empty_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plain: Option<String>,
    #[serde(rename = "type")]
    pub r#type: MeshAccessLogFromDefaultBackendsTcpFormatType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogFromDefaultBackendsTcpFormatJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Format of access logs. Placeholders available on https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage#command-operators
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MeshAccessLogFromDefaultBackendsTcpFormatType {
    Plain,
    Json,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MeshAccessLogFromDefaultBackendsType {
    Tcp,
    File,
    OpenTelemetry,
}

/// TargetRef is a reference to the resource that represents a group of clients.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogFromTargetRef {
    /// Kind of the referenced resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MeshAccessLogFromTargetRefKind>,
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
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MeshAccessLogFromTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
}

/// TargetRef is a reference to the resource the policy takes an effect on. The resource could be either a real store object or virtual resource defined inplace.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogTargetRef {
    /// Kind of the referenced resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MeshAccessLogTargetRefKind>,
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
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MeshAccessLogTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogTo {
    /// Default is a configuration specific to the group of destinations referenced in 'targetRef'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<MeshAccessLogToDefault>,
    /// TargetRef is a reference to the resource that represents a group of destinations.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshAccessLogToTargetRef,
}

/// Default is a configuration specific to the group of destinations referenced in 'targetRef'
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogToDefault {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backends: Option<Vec<MeshAccessLogToDefaultBackends>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogToDefaultBackends {
    /// FileBackend defines configuration for file based access logs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<MeshAccessLogToDefaultBackendsFile>,
    /// Defines an OpenTelemetry logging backend.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openTelemetry")]
    pub open_telemetry: Option<MeshAccessLogToDefaultBackendsOpenTelemetry>,
    /// TCPBackend defines a TCP logging backend.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<MeshAccessLogToDefaultBackendsTcp>,
    #[serde(rename = "type")]
    pub r#type: MeshAccessLogToDefaultBackendsType,
}

/// FileBackend defines configuration for file based access logs
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogToDefaultBackendsFile {
    /// Format of access logs. Placeholders available on https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage#command-operators
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<MeshAccessLogToDefaultBackendsFileFormat>,
    /// Path to a file that logs will be written to
    pub path: String,
}

/// Format of access logs. Placeholders available on https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage#command-operators
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogToDefaultBackendsFileFormat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json: Option<Vec<MeshAccessLogToDefaultBackendsFileFormatJson>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "omitEmptyValues")]
    pub omit_empty_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plain: Option<String>,
    #[serde(rename = "type")]
    pub r#type: MeshAccessLogToDefaultBackendsFileFormatType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogToDefaultBackendsFileFormatJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Format of access logs. Placeholders available on https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage#command-operators
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MeshAccessLogToDefaultBackendsFileFormatType {
    Plain,
    Json,
}

/// Defines an OpenTelemetry logging backend.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogToDefaultBackendsOpenTelemetry {
    /// Attributes can contain placeholders available on https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage#command-operators
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<MeshAccessLogToDefaultBackendsOpenTelemetryAttributes>>,
    /// Body is a raw string or an OTLP any value as described at https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/logs/data-model.md#field-body It can contain placeholders available on https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage#command-operators
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<HashMap<String, serde_json::Value>>,
    /// Endpoint of OpenTelemetry collector. An empty port defaults to 4317.
    pub endpoint: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogToDefaultBackendsOpenTelemetryAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// TCPBackend defines a TCP logging backend.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogToDefaultBackendsTcp {
    /// Address of the TCP logging backend
    pub address: String,
    /// Format of access logs. Placeholders available on https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage#command-operators
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<MeshAccessLogToDefaultBackendsTcpFormat>,
}

/// Format of access logs. Placeholders available on https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage#command-operators
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogToDefaultBackendsTcpFormat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json: Option<Vec<MeshAccessLogToDefaultBackendsTcpFormatJson>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "omitEmptyValues")]
    pub omit_empty_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plain: Option<String>,
    #[serde(rename = "type")]
    pub r#type: MeshAccessLogToDefaultBackendsTcpFormatType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogToDefaultBackendsTcpFormatJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Format of access logs. Placeholders available on https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage#command-operators
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MeshAccessLogToDefaultBackendsTcpFormatType {
    Plain,
    Json,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MeshAccessLogToDefaultBackendsType {
    Tcp,
    File,
    OpenTelemetry,
}

/// TargetRef is a reference to the resource that represents a group of destinations.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshAccessLogToTargetRef {
    /// Kind of the referenced resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MeshAccessLogToTargetRefKind>,
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

/// TargetRef is a reference to the resource that represents a group of destinations.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MeshAccessLogToTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
}

