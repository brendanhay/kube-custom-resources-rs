// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/emissary-ingress/emissary/getambassador.io/v3alpha1/kubernetesserviceresolvers.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// KubernetesServiceResolver tells Ambassador to use Kubernetes Service resources to resolve services. It actually has no spec other than the AmbassadorID.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "getambassador.io", version = "v3alpha1", kind = "KubernetesServiceResolver", plural = "kubernetesserviceresolvers")]
#[kube(namespaced)]
pub struct KubernetesServiceResolverSpec {
    /// AmbassadorID declares which Ambassador instances should pay attention to this resource. If no value is provided, the default is: 
    ///  ambassador_id: - "default"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ambassador_id: Option<Vec<String>>,
}

