// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/l7mp/stunner/stunner.l7mp.io/v1/staticservices.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// Spec defines the behavior of a service.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "stunner.l7mp.io", version = "v1", kind = "StaticService", plural = "staticservices")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct StaticServiceSpec {
    /// Prefixes is a list of IP address prefixes reachable via this route.
    pub prefixes: Vec<String>,
}
