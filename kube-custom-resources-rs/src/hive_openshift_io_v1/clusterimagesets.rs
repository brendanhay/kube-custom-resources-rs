// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/openshift/hive/hive.openshift.io/v1/clusterimagesets.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// ClusterImageSetSpec defines the desired state of ClusterImageSet
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "hive.openshift.io", version = "v1", kind = "ClusterImageSet", plural = "clusterimagesets")]
#[kube(status = "ClusterImageSetStatus")]
#[kube(schema = "disabled")]
pub struct ClusterImageSetSpec {
    /// ReleaseImage is the image that contains the payload to use when installing a cluster.
    #[serde(rename = "releaseImage")]
    pub release_image: String,
}

/// ClusterImageSetStatus defines the observed state of ClusterImageSet
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterImageSetStatus {
}

