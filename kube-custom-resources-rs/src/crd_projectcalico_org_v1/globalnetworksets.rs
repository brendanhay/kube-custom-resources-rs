// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/projectcalico/calico/crd.projectcalico.org/v1/globalnetworksets.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// GlobalNetworkSetSpec contains the specification for a NetworkSet resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "crd.projectcalico.org", version = "v1", kind = "GlobalNetworkSet", plural = "globalnetworksets")]
#[kube(schema = "disabled")]
pub struct GlobalNetworkSetSpec {
    /// The list of IP networks that belong to this set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nets: Option<Vec<String>>,
}

