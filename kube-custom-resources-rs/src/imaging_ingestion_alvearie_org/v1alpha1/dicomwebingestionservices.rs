// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/Alvearie/imaging-ingestion/imaging-ingestion.alvearie.org/v1alpha1/dicomwebingestionservices.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// DicomwebIngestionServiceSpec defines the desired state of DicomwebIngestionService
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "imaging-ingestion.alvearie.org", version = "v1alpha1", kind = "DicomwebIngestionService", plural = "dicomwebingestionservices")]
#[kube(namespaced)]
#[kube(status = "DicomwebIngestionServiceStatus")]
#[kube(schema = "disabled")]
pub struct DicomwebIngestionServiceSpec {
    /// Bucket Config Name
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bucketConfigName")]
    pub bucket_config_name: Option<String>,
    /// Bucket Secret Name
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bucketSecretName")]
    pub bucket_secret_name: Option<String>,
    /// DICOM Event Driven Ingestion Name
    #[serde(rename = "dicomEventDrivenIngestionName")]
    pub dicom_event_driven_ingestion_name: String,
    /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
    /// Image Pull Secrets
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecrets")]
    pub image_pull_secrets: Option<Vec<DicomwebIngestionServiceImagePullSecrets>>,
    /// Provider Name
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerName")]
    pub provider_name: Option<String>,
    /// STOW Service Spec
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stowService")]
    pub stow_service: Option<DicomwebIngestionServiceStowService>,
    /// WADO Service Spec
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "wadoService")]
    pub wado_service: Option<DicomwebIngestionServiceWadoService>,
}

/// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DicomwebIngestionServiceImagePullSecrets {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// STOW Service Spec
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DicomwebIngestionServiceStowService {
    /// Container Concurrency
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i64>,
    /// Image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Max Replicas
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxReplicas")]
    pub max_replicas: Option<i32>,
    /// Min Replicas
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minReplicas")]
    pub min_replicas: Option<i32>,
}

/// WADO Service Spec
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DicomwebIngestionServiceWadoService {
    /// Container Concurrency
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i64>,
    /// Image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Max Replicas
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxReplicas")]
    pub max_replicas: Option<i32>,
    /// Min Replicas
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minReplicas")]
    pub min_replicas: Option<i32>,
}

/// DicomwebIngestionServiceStatus defines the observed state of DicomwebIngestionService
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DicomwebIngestionServiceStatus {
    /// Human-readable message indicating details about current operator phase or error
    pub message: String,
    /// Current phase of the operator
    pub phase: String,
    /// True if all resources are in a ready state and all work is done
    pub ready: bool,
    /// A map of all the secondary resources types and names created for this CR. e.g "Deployment": [ "DeploymentName1", "DeploymentName2" ]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secondaryResources")]
    pub secondary_resources: Option<BTreeMap<String, String>>,
    /// WADO Service External Endpoint
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "wadoServiceExternalEndpoint")]
    pub wado_service_external_endpoint: Option<String>,
    /// WADO Service Internal Endpoint
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "wadoServiceInternalEndpoint")]
    pub wado_service_internal_endpoint: Option<String>,
}
