// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/clusterpedia-io/clusterpedia/cluster.clusterpedia.io/v1alpha2/clustersyncresources.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "cluster.clusterpedia.io", version = "v1alpha2", kind = "ClusterSyncResources", plural = "clustersyncresources")]
pub struct ClusterSyncResourcesSpec {
    #[serde(rename = "syncResources")]
    pub sync_resources: Vec<ClusterSyncResourcesSyncResources>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ClusterSyncResourcesSyncResources {
    pub group: String,
    pub resources: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,
}

