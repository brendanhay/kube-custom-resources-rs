// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/clastix/capsule/capsule.clastix.io/v1beta2/capsuleconfigurations.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// CapsuleConfigurationSpec defines the Capsule configuration.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "capsule.clastix.io", version = "v1beta2", kind = "CapsuleConfiguration", plural = "capsuleconfigurations")]
#[kube(schema = "disabled")]
pub struct CapsuleConfigurationSpec {
    /// Toggles the TLS reconciler, the controller that is able to generate CA and certificates for the webhooks when not using an already provided CA and certificate, or when these are managed externally with Vault, or cert-manager.
    #[serde(rename = "enableTLSReconciler")]
    pub enable_tls_reconciler: bool,
    /// Enforces the Tenant owner, during Namespace creation, to name it using the selected Tenant name as prefix, separated by a dash. This is useful to avoid Namespace name collision in a public CaaS environment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forceTenantPrefix")]
    pub force_tenant_prefix: Option<bool>,
    /// Allows to set the forbidden metadata for the worker nodes that could be patched by a Tenant. This applies only if the Tenant has an active NodeSelector, and the Owner have right to patch their nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeMetadata")]
    pub node_metadata: Option<CapsuleConfigurationNodeMetadata>,
    /// Allows to set different name rather than the canonical one for the Capsule configuration objects, such as webhook secret or configurations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<CapsuleConfigurationOverrides>,
    /// Disallow creation of namespaces, whose name matches this regexp
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protectedNamespaceRegex")]
    pub protected_namespace_regex: Option<String>,
    /// Names of the groups for Capsule users.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userGroups")]
    pub user_groups: Option<Vec<String>>,
}

/// Allows to set the forbidden metadata for the worker nodes that could be patched by a Tenant. This applies only if the Tenant has an active NodeSelector, and the Owner have right to patch their nodes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CapsuleConfigurationNodeMetadata {
    /// Define the annotations that a Tenant Owner cannot set for their nodes.
    #[serde(rename = "forbiddenAnnotations")]
    pub forbidden_annotations: CapsuleConfigurationNodeMetadataForbiddenAnnotations,
    /// Define the labels that a Tenant Owner cannot set for their nodes.
    #[serde(rename = "forbiddenLabels")]
    pub forbidden_labels: CapsuleConfigurationNodeMetadataForbiddenLabels,
}

/// Define the annotations that a Tenant Owner cannot set for their nodes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CapsuleConfigurationNodeMetadataForbiddenAnnotations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denied: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deniedRegex")]
    pub denied_regex: Option<String>,
}

/// Define the labels that a Tenant Owner cannot set for their nodes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CapsuleConfigurationNodeMetadataForbiddenLabels {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denied: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deniedRegex")]
    pub denied_regex: Option<String>,
}

/// Allows to set different name rather than the canonical one for the Capsule configuration objects, such as webhook secret or configurations.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CapsuleConfigurationOverrides {
    /// Defines the Secret name used for the webhook server. Must be in the same Namespace where the Capsule Deployment is deployed.
    #[serde(rename = "TLSSecretName")]
    pub tls_secret_name: String,
    /// Name of the MutatingWebhookConfiguration which contains the dynamic admission controller paths and resources.
    #[serde(rename = "mutatingWebhookConfigurationName")]
    pub mutating_webhook_configuration_name: String,
    /// Name of the ValidatingWebhookConfiguration which contains the dynamic admission controller paths and resources.
    #[serde(rename = "validatingWebhookConfigurationName")]
    pub validating_webhook_configuration_name: String,
}
