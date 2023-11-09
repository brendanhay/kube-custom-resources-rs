// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive Default --derive PartialEq --docs --filename ./crd-catalog/Alvearie/imaging-ingestion/imaging-ingestion.alvearie.org/v1alpha1/dicomeventbridges.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// DicomEventBridgeSpec defines the desired state of DicomEventBridge
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "imaging-ingestion.alvearie.org", version = "v1alpha1", kind = "DicomEventBridge", plural = "dicomeventbridges")]
#[kube(namespaced)]
#[kube(status = "DicomEventBridgeStatus")]
#[kube(schema = "disabled")]
pub struct DicomEventBridgeSpec {
    /// DICOM Event Driven Ingestion Name
    #[serde(rename = "dicomEventDrivenIngestionName")]
    pub dicom_event_driven_ingestion_name: String,
    /// Event Bridge Edge Mailbox. Required when Role is edge.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "edgeMailbox")]
    pub edge_mailbox: Option<String>,
    /// Event Bridge Deployment Spec
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eventBridge")]
    pub event_bridge: Option<DicomEventBridgeEventBridge>,
    /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
    /// Image Pull Secrets
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecrets")]
    pub image_pull_secrets: Option<Vec<DicomEventBridgeImagePullSecrets>>,
    /// Make NATS Connection Secure
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "natsSecure")]
    pub nats_secure: Option<bool>,
    /// NATS Subject Root
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "natsSubjectRoot")]
    pub nats_subject_root: Option<String>,
    /// NATS Token Secret Name
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "natsTokenSecret")]
    pub nats_token_secret: Option<String>,
    /// NATS URL
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "natsUrl")]
    pub nats_url: Option<String>,
    /// Event Bridge Role
    pub role: String,
}

/// Event Bridge Deployment Spec
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DicomEventBridgeEventBridge {
    /// Image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

/// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DicomEventBridgeImagePullSecrets {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// DicomEventBridgeStatus defines the observed state of DicomEventBridge
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DicomEventBridgeStatus {
    /// Human-readable message indicating details about current operator phase or error
    pub message: String,
    /// Current phase of the operator
    pub phase: String,
    /// True if all resources are in a ready state and all work is done
    pub ready: bool,
    /// A map of all the secondary resources types and names created for this CR. e.g "Deployment": [ "DeploymentName1", "DeploymentName2" ]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secondaryResources")]
    pub secondary_resources: Option<BTreeMap<String, String>>,
}

