// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/3scale-ops/marin3r/operator.marin3r.3scale.net/v1alpha1/discoveryservices.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// DiscoveryServiceSpec defines the desired state of DiscoveryService
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "operator.marin3r.3scale.net", version = "v1alpha1", kind = "DiscoveryService", plural = "discoveryservices")]
#[kube(namespaced)]
#[kube(status = "DiscoveryServiceStatus")]
#[kube(schema = "disabled")]
pub struct DiscoveryServiceSpec {
    /// Debug enables debugging log level for the discovery service controllers. It is safe to use since secret data is never shown in the logs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    /// Image holds the image to use for the discovery service Deployment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// MetricsPort is the port where metrics are served. Defaults to 8383.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metricsPort")]
    pub metrics_port: Option<i32>,
    /// PKIConfig has configuration for the PKI that marin3r manages for the different certificates it requires
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pkiConfg")]
    pub pki_confg: Option<DiscoveryServicePkiConfg>,
    /// PriorityClass to assign the discovery service Pod to
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podPriorityClass")]
    pub pod_priority_class: Option<String>,
    /// ProbePort is the port where healthz endpoint is served. Defaults to 8384.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "probePort")]
    pub probe_port: Option<i32>,
    /// Resources holds the Resource Requirements to use for the discovery service Deployment. When not set it defaults to no resource requests nor limits. CPU and Memory resources are supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<DiscoveryServiceResources>,
    /// ServiceConfig configures the way the DiscoveryService endpoints are exposed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceConfig")]
    pub service_config: Option<DiscoveryServiceServiceConfig>,
    /// XdsServerPort is the port where the xDS server listens. Defaults to 18000.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "xdsServerPort")]
    pub xds_server_port: Option<i32>,
}

/// PKIConfig has configuration for the PKI that marin3r manages for the different certificates it requires
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiscoveryServicePkiConfg {
    /// CertificateOptions specifies options to generate the server certificate used both for the xDS server and the mutating webhook server.
    #[serde(rename = "rootCertificateAuthority")]
    pub root_certificate_authority: DiscoveryServicePkiConfgRootCertificateAuthority,
    /// CertificateOptions specifies options to generate the server certificate used both for the xDS server and the mutating webhook server.
    #[serde(rename = "serverCertificate")]
    pub server_certificate: DiscoveryServicePkiConfgServerCertificate,
}

/// CertificateOptions specifies options to generate the server certificate used both for the xDS server and the mutating webhook server.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiscoveryServicePkiConfgRootCertificateAuthority {
    pub duration: String,
    #[serde(rename = "secretName")]
    pub secret_name: String,
}

/// CertificateOptions specifies options to generate the server certificate used both for the xDS server and the mutating webhook server.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiscoveryServicePkiConfgServerCertificate {
    pub duration: String,
    #[serde(rename = "secretName")]
    pub secret_name: String,
}

/// Resources holds the Resource Requirements to use for the discovery service Deployment. When not set it defaults to no resource requests nor limits. CPU and Memory resources are supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiscoveryServiceResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims, that are used by this container. 
    ///  This is an alpha field and requires enabling the DynamicResourceAllocation feature gate. 
    ///  This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<DiscoveryServiceResourcesClaims>>,
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiscoveryServiceResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of the Pod where this field is used. It makes that resource available inside a container.
    pub name: String,
}

/// ServiceConfig configures the way the DiscoveryService endpoints are exposed
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiscoveryServiceServiceConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ServiceType is an enum with the available discovery service Service types
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// DiscoveryServiceStatus defines the observed state of DiscoveryService
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiscoveryServiceStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deploymentName")]
    pub deployment_name: Option<String>,
    /// DeploymentStatus is the most recently observed status of the Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deploymentStatus")]
    pub deployment_status: Option<DiscoveryServiceStatusDeploymentStatus>,
}

/// DeploymentStatus is the most recently observed status of the Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiscoveryServiceStatusDeploymentStatus {
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this deployment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availableReplicas")]
    pub available_replicas: Option<i32>,
    /// Count of hash collisions for the Deployment. The Deployment controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ReplicaSet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "collisionCount")]
    pub collision_count: Option<i32>,
    /// Represents the latest available observations of a deployment's current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The generation observed by the deployment controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// readyReplicas is the number of pods targeted by this Deployment with a Ready Condition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyReplicas")]
    pub ready_replicas: Option<i32>,
    /// Total number of non-terminated pods targeted by this deployment (their labels match the selector).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Total number of unavailable pods targeted by this deployment. This is the total number of pods that are still required for the deployment to have 100% available capacity. They may either be pods that are running but not yet available or pods that still have not been created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unavailableReplicas")]
    pub unavailable_replicas: Option<i32>,
    /// Total number of non-terminated pods targeted by this deployment that have the desired template spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatedReplicas")]
    pub updated_replicas: Option<i32>,
}
