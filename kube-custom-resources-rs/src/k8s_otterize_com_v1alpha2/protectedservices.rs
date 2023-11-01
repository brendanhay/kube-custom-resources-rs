// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --auto --filename ./crd-catalog/otterize/helm-charts/k8s.otterize.com/v1alpha2/protectedservices.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// ProtectedServiceSpec defines the desired state of ProtectedService
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "k8s.otterize.com", version = "v1alpha2", kind = "ProtectedService", plural = "protectedservices")]
#[kube(namespaced)]
#[kube(status = "ProtectedServiceStatus")]
pub struct ProtectedServiceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ProtectedServiceStatus defines the observed state of ProtectedService
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ProtectedServiceStatus {
}

