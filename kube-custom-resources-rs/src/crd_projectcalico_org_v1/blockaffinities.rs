// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/projectcalico/calico/crd.projectcalico.org/v1/blockaffinities.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// BlockAffinitySpec contains the specification for a BlockAffinity resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "crd.projectcalico.org", version = "v1", kind = "BlockAffinity", plural = "blockaffinities")]
pub struct BlockAffinitySpec {
    pub cidr: String,
    /// Deleted indicates that this block affinity is being deleted. This field is a string for compatibility with older releases that mistakenly treat this field as a string.
    pub deleted: String,
    pub node: String,
    pub state: String,
}

