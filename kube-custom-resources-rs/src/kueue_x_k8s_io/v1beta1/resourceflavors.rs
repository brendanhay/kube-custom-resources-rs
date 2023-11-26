// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/kueue/kueue.x-k8s.io/v1beta1/resourceflavors.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// ResourceFlavorSpec defines the desired state of the ResourceFlavor
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kueue.x-k8s.io", version = "v1beta1", kind = "ResourceFlavor", plural = "resourceflavors")]
#[kube(schema = "disabled")]
pub struct ResourceFlavorSpec {
    /// nodeLabels are labels that associate the ResourceFlavor with Nodes that have the same labels. When a Workload is admitted, its podsets can only get assigned ResourceFlavors whose nodeLabels match the nodeSelector and nodeAffinity fields. Once a ResourceFlavor is assigned to a podSet, the ResourceFlavor's nodeLabels should be injected into the pods of the Workload by the controller that integrates with the Workload object. 
    ///  nodeLabels can be up to 8 elements.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeLabels")]
    pub node_labels: Option<BTreeMap<String, String>>,
    /// nodeTaints are taints that the nodes associated with this ResourceFlavor have. Workloads' podsets must have tolerations for these nodeTaints in order to get assigned this ResourceFlavor during admission. 
    ///  An example of a nodeTaint is cloud.provider.com/preemptible="true":NoSchedule 
    ///  nodeTaints can be up to 8 elements.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeTaints")]
    pub node_taints: Option<Vec<ResourceFlavorNodeTaints>>,
    /// tolerations are extra tolerations that will be added to the pods admitted in the quota associated with this resource flavor. 
    ///  An example of a toleration is cloud.provider.com/preemptible="true":NoSchedule 
    ///  tolerations can be up to 8 elements.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<ResourceFlavorTolerations>>,
}

/// The node this Taint is attached to has the "effect" on any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResourceFlavorNodeTaints {
    /// Required. The effect of the taint on pods that do not tolerate the taint. Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
    pub effect: String,
    /// Required. The taint key to be applied to a node.
    pub key: String,
    /// TimeAdded represents the time at which the taint was added. It is only written for NoExecute taints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeAdded")]
    pub time_added: Option<String>,
    /// The taint value corresponding to the taint key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResourceFlavorTolerations {
    /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
