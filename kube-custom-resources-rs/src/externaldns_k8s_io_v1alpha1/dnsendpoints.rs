// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/k8gb-io/k8gb/externaldns.k8s.io/v1alpha1/dnsendpoints.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// DNSEndpointSpec defines the desired state of DNSEndpoint
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "externaldns.k8s.io", version = "v1alpha1", kind = "DNSEndpoint", plural = "dnsendpoints")]
#[kube(namespaced)]
#[kube(status = "DNSEndpointStatus")]
pub struct DNSEndpointSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<DNSEndpointEndpoints>>,
}

/// Endpoint is a high-level way of a connection between a service and an IP
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DNSEndpointEndpoints {
    /// The hostname of the DNS record
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsName")]
    pub dns_name: Option<String>,
    /// Labels stores labels defined for the Endpoint
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// ProviderSpecific stores provider specific config
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerSpecific")]
    pub provider_specific: Option<Vec<DNSEndpointEndpointsProviderSpecific>>,
    /// TTL for the record
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "recordTTL")]
    pub record_ttl: Option<i64>,
    /// RecordType type of record, e.g. CNAME, A, SRV, TXT etc
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "recordType")]
    pub record_type: Option<String>,
    /// Identifier to distinguish multiple records with the same name and type (e.g. Route53 records with routing policies other than 'simple')
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "setIdentifier")]
    pub set_identifier: Option<String>,
    /// The targets the DNS record points to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

/// ProviderSpecificProperty holds the name and value of a configuration which is specific to individual DNS providers
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DNSEndpointEndpointsProviderSpecific {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// DNSEndpointStatus defines the observed state of DNSEndpoint
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DNSEndpointStatus {
    /// The generation observed by the external-dns controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

