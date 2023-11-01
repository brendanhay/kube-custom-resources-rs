// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/kubernetes-sigs/cluster-api/ipam.cluster.x-k8s.io/v1alpha1/ipaddresses.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// IPAddressSpec is the desired state of an IPAddress.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "ipam.cluster.x-k8s.io", version = "v1alpha1", kind = "IPAddress", plural = "ipaddresses")]
#[kube(namespaced)]
pub struct IPAddressSpec {
    /// Address is the IP address.
    pub address: String,
    /// ClaimRef is a reference to the claim this IPAddress was created for.
    #[serde(rename = "claimRef")]
    pub claim_ref: IPAddressClaimRef,
    /// Gateway is the network gateway of the network the address is from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// PoolRef is a reference to the pool that this IPAddress was created from.
    #[serde(rename = "poolRef")]
    pub pool_ref: IPAddressPoolRef,
    /// Prefix is the prefix of the address.
    pub prefix: i64,
}

/// ClaimRef is a reference to the claim this IPAddress was created for.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct IPAddressClaimRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// PoolRef is a reference to the pool that this IPAddress was created from.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct IPAddressPoolRef {
    /// APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroup")]
    pub api_group: Option<String>,
    /// Kind is the type of resource being referenced
    pub kind: String,
    /// Name is the name of resource being referenced
    pub name: String,
}

