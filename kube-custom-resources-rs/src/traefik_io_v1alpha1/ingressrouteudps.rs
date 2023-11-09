// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/traefik/traefik/traefik.io/v1alpha1/ingressrouteudps.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// IngressRouteUDPSpec defines the desired state of a IngressRouteUDP.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "traefik.io", version = "v1alpha1", kind = "IngressRouteUDP", plural = "ingressrouteudps")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct IngressRouteUDPSpec {
    /// EntryPoints defines the list of entry point names to bind to. Entry points have to be configured in the static configuration. More info: https://doc.traefik.io/traefik/v3.0/routing/entrypoints/ Default: all.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "entryPoints")]
    pub entry_points: Option<Vec<String>>,
    /// Routes defines the list of routes.
    pub routes: Vec<IngressRouteUDPRoutes>,
}

/// RouteUDP holds the UDP route configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IngressRouteUDPRoutes {
    /// Services defines the list of UDP services.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<IngressRouteUDPRoutesServices>>,
}

/// ServiceUDP defines an upstream UDP service to proxy traffic to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IngressRouteUDPRoutesServices {
    /// Name defines the name of the referenced Kubernetes Service.
    pub name: String,
    /// Namespace defines the namespace of the referenced Kubernetes Service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// NativeLB controls, when creating the load-balancer, whether the LB's children are directly the pods IPs or if the only child is the Kubernetes Service clusterIP. The Kubernetes Service itself does load-balance to the pods. By default, NativeLB is false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nativeLB")]
    pub native_lb: Option<bool>,
    /// Port defines the port of a Kubernetes Service. This can be a reference to a named port.
    pub port: IntOrString,
    /// Weight defines the weight used when balancing requests between multiple Kubernetes Service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

