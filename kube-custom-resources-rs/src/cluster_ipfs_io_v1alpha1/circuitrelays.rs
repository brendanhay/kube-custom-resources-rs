// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/ipfs-cluster/ipfs-operator/cluster.ipfs.io/v1alpha1/circuitrelays.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// CircuitRelaySpec Defines a specification for the RelayCircuit launched by Kubernetes.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cluster.ipfs.io", version = "v1alpha1", kind = "CircuitRelay", plural = "circuitrelays")]
#[kube(namespaced)]
#[kube(status = "CircuitRelayStatus")]
#[kube(schema = "disabled")]
pub struct CircuitRelaySpec {
    /// SwarmKeyRef points to a multicodec-encoded v1 PSK stored within a secret somewhere.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "swarmKeyRef")]
    pub swarm_key_ref: Option<CircuitRelaySwarmKeyRef>,
}

/// SwarmKeyRef points to a multicodec-encoded v1 PSK stored within a secret somewhere.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CircuitRelaySwarmKeyRef {
    #[serde(rename = "keyName")]
    pub key_name: String,
    #[serde(rename = "secretName")]
    pub secret_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CircuitRelayStatus {
    /// This is intended to mimic peer.AddrInfo.
    #[serde(rename = "addrInfo")]
    pub addr_info: CircuitRelayStatusAddrInfo,
}

/// This is intended to mimic peer.AddrInfo.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CircuitRelayStatusAddrInfo {
    pub addrs: Vec<String>,
    pub id: String,
}

