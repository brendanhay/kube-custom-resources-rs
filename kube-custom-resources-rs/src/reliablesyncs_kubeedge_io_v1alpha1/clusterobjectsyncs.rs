// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/kubeedge/kubeedge/reliablesyncs.kubeedge.io/v1alpha1/clusterobjectsyncs.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// ObjectSyncSpec stores the details of objects that persist to the edge.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "reliablesyncs.kubeedge.io", version = "v1alpha1", kind = "ClusterObjectSync", plural = "clusterobjectsyncs")]
#[kube(status = "ClusterObjectSyncStatus")]
pub struct ClusterObjectSyncSpec {
    /// ObjectAPIVersion is the APIVersion of the object that was successfully persist to the edge node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "objectAPIVersion")]
    pub object_api_version: Option<String>,
    /// ObjectType is the kind of the object that was successfully persist to the edge node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "objectKind")]
    pub object_kind: Option<String>,
    /// ObjectName is the name of the object that was successfully persist to the edge node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "objectName")]
    pub object_name: Option<String>,
}

/// ObjectSyncStatus stores the resourceversion of objects that persist to the edge.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ClusterObjectSyncStatus {
    /// ObjectResourceVersion is the resourceversion of the object that was successfully persist to the edge node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "objectResourceVersion")]
    pub object_resource_version: Option<String>,
}

