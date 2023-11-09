// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/kumahq/kuma/kuma.io/v1alpha1/meshfaultinjections.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// Spec is the specification of the Kuma MeshFaultInjection resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kuma.io", version = "v1alpha1", kind = "MeshFaultInjection", plural = "meshfaultinjections")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct MeshFaultInjectionSpec {
    /// From list makes a match between clients and corresponding configurations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<MeshFaultInjectionFrom>>,
    /// TargetRef is a reference to the resource the policy takes an effect on. The resource could be either a real store object or virtual resource defined inplace.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshFaultInjectionTargetRef,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshFaultInjectionFrom {
    /// Default is a configuration specific to the group of destinations referenced in 'targetRef'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<MeshFaultInjectionFromDefault>,
    /// TargetRef is a reference to the resource that represents a group of destinations.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshFaultInjectionFromTargetRef,
}

/// Default is a configuration specific to the group of destinations referenced in 'targetRef'
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshFaultInjectionFromDefault {
    /// Http allows to define list of Http faults between dataplanes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<Vec<MeshFaultInjectionFromDefaultHttp>>,
}

/// FaultInjection defines the configuration of faults between dataplanes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshFaultInjectionFromDefaultHttp {
    /// Abort defines a configuration of not delivering requests to destination service and replacing the responses from destination dataplane by predefined status code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abort: Option<MeshFaultInjectionFromDefaultHttpAbort>,
    /// Delay defines configuration of delaying a response from a destination
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<MeshFaultInjectionFromDefaultHttpDelay>,
    /// ResponseBandwidth defines a configuration to limit the speed of responding to the requests
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseBandwidth")]
    pub response_bandwidth: Option<MeshFaultInjectionFromDefaultHttpResponseBandwidth>,
}

/// Abort defines a configuration of not delivering requests to destination service and replacing the responses from destination dataplane by predefined status code
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshFaultInjectionFromDefaultHttpAbort {
    /// HTTP status code which will be returned to source side
    #[serde(rename = "httpStatus")]
    pub http_status: i32,
    /// Percentage of requests on which abort will be injected, has to be either int or decimal represented as string.
    pub percentage: IntOrString,
}

/// Delay defines configuration of delaying a response from a destination
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshFaultInjectionFromDefaultHttpDelay {
    /// Percentage of requests on which delay will be injected, has to be either int or decimal represented as string.
    pub percentage: IntOrString,
    /// The duration during which the response will be delayed
    pub value: String,
}

/// ResponseBandwidth defines a configuration to limit the speed of responding to the requests
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshFaultInjectionFromDefaultHttpResponseBandwidth {
    /// Limit is represented by value measure in gbps, mbps, kbps or bps, e.g. 10kbps
    pub limit: String,
    /// Percentage of requests on which response bandwidth limit will be either int or decimal represented as string.
    pub percentage: IntOrString,
}

/// TargetRef is a reference to the resource that represents a group of destinations.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshFaultInjectionFromTargetRef {
    /// Kind of the referenced resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MeshFaultInjectionFromTargetRefKind>,
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshFaultInjectionFromTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
}

/// TargetRef is a reference to the resource the policy takes an effect on. The resource could be either a real store object or virtual resource defined inplace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshFaultInjectionTargetRef {
    /// Kind of the referenced resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MeshFaultInjectionTargetRefKind>,
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshFaultInjectionTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
}

